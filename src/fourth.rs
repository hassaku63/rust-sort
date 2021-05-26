use super::SortOrder;
use std::cmp::Ordering;

const PARALLEL_THRESHOLD: usize = 4096;

pub fn sort<T: Ord + Send>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    match *order {
        SortOrder::Ascending  => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

// クロージャは「匿名の型」として扱うので、同じ引数・戻り値で2つのクロージャを定義しても両者は異なる型になる仕様。
// 例えば、以下の x1, x2 は異なる型。
// 
//   let x1 = |a, b| a.cmp/(&b);
//   let x2 = |a, b| a.cmp/(&b);
//
// なので、comparator のところに具体的な型は書けない。
// -> クロージャを引数にとる場合はジェネリクスにする必要がある
pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
    where T: Send,
        F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!("The length of x is not a power of two. x.len(): {}", x.len()))
    }
}

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where T: Send,
        F: Sync + Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;

        // x の可変参照を2つ以上作らせない実装（NG例）
        // let first = &mut x[..mid_point];  // first, second で事前分割しても、x の可変参照はこの時点で作られてしまうので、
        // let second  = &mut x[mid_point..];  // （承前）second のところで "cannot borrow as mutable" で弾かれる

        // x の可変参照を2つ以上作らせない実装（OK例）
        let (first, second) = x.split_at_mut(mid_point);  // mid_point を堺にした2つの可変参照に分割して、それぞれ first, second に束縛

        if mid_point >= PARALLEL_THRESHOLD {
            // 要素数がしきい値以上なら並列実行する。
            // しきい値はスレッド作成のオーバーヘッドとの兼ね合い
            rayon::join(
                || do_sort(first, true, comparator),
                || do_sort(second, false, comparator)
            );
            // rayon_core::join
            // pub fn join<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
            // where
            //     A: FnOnce<(), Output = RA> + Send,
            //     B: FnOnce<(), Output = RB> + Send,
            //     RA: Send,
            //     RB: Send,
            // A, B はクロージャの型で、それぞれの戻り値が RA, RB ということらしい。
            // Send は謎。
            // mutable で参照渡ししている x は大丈夫なのか？という疑問はあるが、
            // 分割統治のアルゴリズムなので再帰の深い場所から上がってくるだけだし多重更新みたいな問題は起きないだろう...と、理解している
        } else {
            do_sort(first, true, comparator);
            do_sort(second, false, comparator);
        }
        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where T: Send,
        F: Sync + Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        let (first, second) = x.split_at_mut(mid_point);
        if mid_point >= PARALLEL_THRESHOLD {
            rayon::join(
                || sub_sort(first, forward, comparator),
                || sub_sort(second, forward, comparator)
            );
        } else {
            sub_sort(first, forward, comparator);
            sub_sort(second, forward, comparator);
        }
    }
}

fn compare_and_swap<T, F>(x: &mut[T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };

    let mid_point = x.len() / 2;

    for i in 0..mid_point {
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            // x[i] = x[mid_point+i];
            // x[mid_point] = x[i];
            x.swap(i, mid_point+i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{sort, sort_by};
    use crate::SortOrder::{Ascending as Asc, Descending as Desc};
    use crate::utils::{
        new_u32_vec,
        // is_sorted_ascending,
        // is_sorted_descending,
        is_sorted
    };

    #[test]
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        sort(&mut x, &Asc);

        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        sort(&mut x, &Desc);

        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];

        sort(&mut x, &Asc);

        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];

        sort(&mut x, &Desc);

        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and","Rust", "GC"]);
    }

    // コンパイラが型エラーを検出してくれる例
    // #[test]
    // fn sort_f64_mixed() {
    //     let mut x = vec![10, 30, "a", "b"];
    //     sort(&mut x, true);
    // }

    // PartialEq, Debug の2つは "導出" というコード自動生成に対応しているため、
    // 自分でトレイトを実装する必要がなく、 derive に記述するだけでOK
    #[derive(PartialEq, Debug)]
    struct Student {
        first_name: String,
        last_name: String,
        age: u8,
    }

    impl Student {
        fn new (first_name: &str, last_name: &str, age: u8) -> Self {
            // 構造体 Student の初期化. Self は impl 対象の型 (Student) の別名
            Self {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age,  // フィールド名と変数が同じ場合は省略可能。 ts に近い
            }
        }
    }

    // Vector を == で比較するには、要素である Student が PartialEq トレイトを実装する必要がある。
    // ちゃんと書くなら以下のようになるが、 #[derive] attribute をつければOK。
    // impl PartialEq for Student {
    //     fn eq(&self, other: &Self) -> bool {
    //         self.first_name == other.first_name
    //             && self.last_name == other.last_name
    //             && self.age == other.age
    //     }
    // }

    #[test]
    fn sort_students_by_age_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        assert_eq!(
            sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
            Ok(())
        );
        // 型注釈付きで書く場合はこうなる↓
        // sort_by(|a: &&Student, b: &&Student| -> std::cmp::Ordering {
        //     a.age.cmp(&b.age)
        // });

        assert_eq!(x, expected);
    }

    #[test]
    fn sort_students_by_name_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(
            sort_by(&mut x, 
                &|a, b| a.last_name.cmp(&b.last_name)
                    .then_with(|| a.first_name.cmp(&b.first_name))
            ),
            Ok(())
        );

        assert_eq!(x, expected);
    }

    #[test]
    fn sort_u32_large() {
        {
            let mut x =  new_u32_vec(65536);

            assert_eq!(
                sort(&mut x, &Asc),
                Ok(()));
            
            assert!(is_sorted(&x, &Asc));
        }
        {
            let mut x =  new_u32_vec(65536);

            assert_eq!(
                sort(&mut x, &Desc),
                Ok(()));
            
            assert!(is_sorted(&x, &Desc));
        }
    }
}