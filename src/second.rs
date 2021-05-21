pub fn sort(x: &mut [u32], up: bool) {
    // [u32] はスライス。Vec とはまた異なるらしい
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap(x: &mut[u32], up: bool) {
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

    #[test]
    fn sort_u32_ascending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];

        sort(&mut x, true);

        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];

        sort(&mut x, false);

        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        let mut x = vec!["Rust", "is", "fast", "and", "momory-efficient", "with", "no", "GC"];

        sort(&mut x, true);

        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec!["Rust", "is", "fast", "and", "momory-efficient", "with", "no", "GC"];

        sort(&mut x, false);

        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and","Rust", "GC"]);
    }
}