use super::SortOrder;
use std::cmp::Ordering;


pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
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
    where F: Fn(&T, &T) -> Ordering
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!("The length of x is not a power of two. x.len(): {}", x.len()))
    }
}

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
    // Generics を付けただけでは、以下のようなエラーが出てしまう。
    // > binary operation `>` cannot be applied to type `T`
    // 
    //
    // [u32] はスライス。Vec とはまた異なるらしい
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true, comparator);
        do_sort(&mut x[mid_point..], false, comparator);
        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward, comparator);
        sub_sort(&mut x[mid_point..], forward, comparator);
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
        is_sorted_ascending,
        is_sorted_descending,
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