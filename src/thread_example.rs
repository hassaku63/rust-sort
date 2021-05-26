use std::{time, thread};


#[allow(dead_code)]
fn main() {
    let n1 = 1200;
    let n2 = 1000;

    // spawn で小スレットを立ち上げる
    let child = thread::spawn(move || {
        heavy("child", n2)
    });

    let s1 = heavy("main", n1);

    match child.join() { 
        Ok(s2) => println!("{}, {}", s1, s2),
        Err(e) => println!("error: {:?}", e),
    }
}

#[allow(dead_code)]
fn heavy(name: &str, n: u64) -> u64 {
    println!("{}: started", name);

    // 重たい処理の代用
    thread::sleep(time::Duration::from_millis(n));

    let sum = (1..n).sum();
    println!("{}: ended", name);
    sum
}

#[cfg(test)]
mod test {
    use crate::thread_example;

    #[test]
    fn test_run() {
        //　このレポジトリは bin クレートではないので、 test で実行させてみる
        thread_example::main();
    }
}