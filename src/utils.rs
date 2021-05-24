use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // TODO: from_seed の書き方と、型定義の Seed の定義の書き方がわからないので調べる
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    let mut v = Vec::with_capacity(n);

    for _ in 0..n {
        v.push(rng.sample(&Standard));
    }

    v
}