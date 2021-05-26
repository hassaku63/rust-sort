use num_cpus;

use rust_sort::SortOrder;
// マルチスレッド非対応の sort
use rust_sort::third::sort as seq_sort;
// マルチスレッド対応の sort
use rust_sort::fourth::sort as par_sort;
use rust_sort::utils::{is_sorted, new_u32_vec};

use std::{env, f64};
use std::str::FromStr;
use std::time::Instant;

fn main() {
    // コマンドライン引数の1番目をパース
    if let Some(n) = env::args().nth(1) {
        let bits = u32::from_str(&n).expect("error parsing argument");

        // sort 実行
        run_sorts(bits);
    } else {
        // コマンドライン引数の指定がない場合は help を表示
        eprintln!(
            "Usage: {} <number of elements in bits>",
            env::args().nth(0).unwrap());
    }
    std::process::exit(1);
}

/// ビット数から、データの要素数を決めてソート (sqeuencial, parallel) を実行する
fn run_sorts(bits: u32) {
    let len = 2.0_f64.powi(bits as i32) as usize;

    println!(
        "sorting {} integers ({:.1} MB)",
        len,
        (len * std::mem::size_of::<u32>()) as f64 / 1024.0 / 1024.0
    );

    println!(
        "cpu info: {} physical cores, {} logical cores",
        num_cpus::get_physical(),
        num_cpus::get()
    );

    // 順次 sort
    let seq_duration = timed_sort(&seq_sort, len ,"seq_sort");

    // 並列 sort
    let par_duration = timed_sort(&par_sort, len, "par_sort");

    println!("speed up: {:.2}x", seq_duration / par_duration);
}

fn timed_sort<F>(sorter: &F, len: usize, name: &str) -> f64 
    where F: Fn(&mut [u32], &SortOrder) -> Result<(), String>,
{
    let mut x = new_u32_vec(len);

    let start = Instant::now();
    sorter(&mut x, &SortOrder::Ascending).expect("Filed to sort.");
    let dur = start.elapsed();

    let nano_secs = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;

    println!(
        "{}: sorted {} integers in {} seconds",
        name, len, nano_secs / 1e9
    );

    assert!(is_sorted(&x, &SortOrder::Ascending));

    nano_secs
}