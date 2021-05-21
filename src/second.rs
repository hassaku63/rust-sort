use super::SortOrder;


pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) {
    match *order {
        SortOrder::Ascending =>  do_sort(x, true),
        SortOrder::Descending => do_sort(x, false),
    }
}

fn do_sort<T: Ord>(x: &mut [T], up: bool) {
    // Generics を付けただけでは、以下のようなエラーが出てしまう。
    // > binary operation `>` cannot be applied to type `T`
    // 
    //
    // [u32] はスライス。Vec とはまた異なるらしい
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true);
        do_sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T: Ord>(x: &mut[T], up: bool) {
    let mid_point = x.len() / 2;

    for i in 0..mid_point {
        if (x[i] > x[mid_point+i]) == up {
            // x[i] = x[mid_point+i];
            // x[mid_point] = x[i];
            x.swap(i, mid_point+i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use crate::SortOrder::{Ascending as Asc, Descending as Desc};

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
}