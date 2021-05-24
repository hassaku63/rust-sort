use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;
use crate::SortOrder;
use crate::SortOrder::{Ascending as Asc, Descending as Desc};

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // TODO: from_seed の書き方と、型定義の Seed の定義の書き方がわからないので調べる
    let rng = Pcg64Mcg::from_seed([0; 16]);

    // let mut v = Vec::with_capacity(n);
    // for _ in 0..n {
    //     v.push(rng.sample(&Standard));
    // }
    // v

    // iterator, collector を使った実装に切り替え
    //
    // # take(usize)
    // Creates an iterator that yields its first n elements.
    // take() is often used with an infinite iterator, to make it finite
    // 
    // # collect()
    // Transforms an iterator into a collection.
    // 
    // sample_iter は無限に値を返すので、 take() を挟んで有限個にする必要がある
    // collect でコレクション型にする
    // 
    // 疑問: 戻りの型は変えなくて良いのか??
    // 
    rng.sample_iter(&Standard)
        .take(n)
        .collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    // windows で reduce や畳み込み的な計算が簡単にできる。
    // ↓の例では 2 要素の窓
    x.windows(2)
        .all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2)
        .all(|pair| pair[1] <= pair[0])
}

pub fn is_sorted<T: Ord>(x: &[T], order: &SortOrder) -> bool {
    match *order {
        Asc => 
            x.windows(2).all(|pair| pair[0] <= pair[1]),
        Desc => 
            x.windows(2).all(|pair| pair[1] <= pair[0]),
    }
}