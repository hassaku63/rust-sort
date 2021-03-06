# README

出典: [実践Rust入門　[言語仕様から開発手法まで]](https://www.amazon.co.jp/dp/B07QVQ7RDG)

Bitonic-sort の実装

学習ログ: [Zenn.dev scrap - Rust に入門した その2](https://zenn.dev/hassaku63/scraps/453a3e4da99ee20)

## Memo

雑多にメモする

### step 2

文字列型を受け入れるためにジェネリックを追加すると、以下のようなエラーが

```plain
$ cargo check
    Checking rust-sort v0.1.0 (/path/to/project/rust-sort)
error[E0369]: binary operation `>` cannot be applied to type `T`
  --> src/second.rs:24:18
   |
24 |         if (x[i] > x[mid_point+i]) == up {
   |             ---- ^ -------------- T
   |             |
   |             T
   |
help: consider restricting type parameter `T`
   |
20 | fn compare_and_swap<T: std::cmp::PartialOrd>(x: &mut[T], up: bool) {
   |                      ^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
error: could not compile `rust-sort`
```

比較のオペレータ ">" が、型 `T` には適用できないと言っている。

実装的には「比較可能な型であれば OK」と言えるので、これに従うトレイト境界を作ってやればOK。
エラーメッセージには `std::cmp::PartialOrd` というトレイトが示されているので、これのドキュメントをを見に行く。

https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html

`PartialOrd` は半順序という意味らしい。例えば `f64` はこのトレイトを実装している。このトレイト**のみ**を実装する型では、**NaN のような他の値との大小が定まらない値がある**。

`Ord` = 全順序というトレイトもあるぽい。例えば、`u32` や文字列型はこの `Ord` トレイトを実装している。

半順序／全順序 という言葉は数学上の定義があるので、それを確認。

[wikipedia - 順序集合](https://ja.wikipedia.org/wiki/%E9%A0%86%E5%BA%8F%E9%9B%86%E5%90%88)

ものすごーくざっくり解釈すると、「比較不可能なケースを許容する」のが半順序 = `PartialOrd` であり、「どの要素間でも比較が可能」なのが全順序 = `Ord` ということになりそう。

今回のソートに関して言えば、半順序では `NaN` の扱いで問題が出る。例えば、`f64` 型では `NaN` が出てきた場合その比較は必ず `false` になってしまうので、ソート済み数列の定義を満たせない。よって、ここで実装するライブラリクレートは全順序 `Ord` を採用する。

このように、step2 の時点では半順序のみ実装した型 (f64 など) をサポートしていない。

次の step3 では半順序のみを実装した型でもソート対応できるようにしていく。

### step 3

ソートの第2引数を enum 型にすることで、より可読性を上げる。

クロージャを導入することで、半順序のみを実装した型でもソートに対応できるようにする。

例えば、`f64` 型では `NaN` の扱いでのみ問題があると判明している。

このライブラリクレートの仕様として、 _`NaN` は最も大きい値として扱う_ と取り決めてしまえば、とりあえずソートは可能になる。これをクロージャで実装する。

### step 4

ソートを並列化する。Bitonic sort のアルゴリズムは分割統治のアプローチであるため並列化が可能。

[Rayon](https://crates.io/crates/rayon) を利用する。

`rayon::join` で 2つのタスクを待ち合わせることができるので、 `do_sort` の部分を `rayon::join` でラップすればよいが、そのままでは動かない。

`do_sort` の引数である、`x` と `comparator` にそれぞれマルチスレッド対応のトレイト境界を追加する必要がある（これはトレイト境界を付けない状態でコンパイラにチェックさせればサジェストしてくれる）。

x には `Send` トレイト、 comparator には `Sync` トレイトが必要。

#### Send trait

今回はソート対象の配列 x に対して Send のトレイト境界を付けた。

こうすることで、この型の値はスレッド間で安全に受け渡し可能であることを保証する。

おそらくだが、今回のアルゴリズムだったから素直にこのトレイト境界を付けられたのだと思われる。
めちゃくちゃなコードを書いてもトレイトつければOK、とかそういう話ではないはず...。

Bitonic sort は分割統治のアルゴリズムで、より細かいスライスに噛み砕いていく方向で再帰が深くなること、
そして分割の元となった再帰の上位スタックでは `rayon::join` を使って子（つまり2分割した子タスク）の終了を待っている。

この2つが噛み合って、 `&x` が複数のスレッドから同時更新されても大丈夫（同時に同じ場所を更新することが起きえない）、という保証ができている、という話なんだと思われる。適当にトレイトつければいいのではなく、アルゴリズムやデータの持ち方、渡し方などでプログラマが安全性を保証して、その上で初めて `Send` をつけてもよい、ということなんではないか（誰か、詳しい人が見てたら訂正してほしい...）

#### Sync trait

スッと入ってこなかったので、説明文を引用。

>ある型がSyncを実装している場合、この型の値は共有された参照を通じて複数のスレッドから並列に使われたとしても、必ずメモリ安全であるとを意味します
> κeen,河野 達也,小松 礼人. 実践Rust入門　[言語仕様から開発手法まで] (Japanese Edition) (Kindle の位置No.3400-3402). Kindle 版. 

クロージャの型に付けるトレイトで「値」とはなにを指すのだろうか？？

クロージャの型なので値はいわゆる「インスタンス」的なものだと思われる。これがメモリ安全であるということは、どういうことか？？

例えば大域変数に依存するなど（実質的に）内部状態を持ってしまっており、リエントラントでないコードである、といった可能性を否定している...ということだろうか？（_リエントラントである ＝ スレッドセーフである_ の式は必ずしも成り立たない、ってどこかで見たような気がするけれど、一次情報を思い出せない）

プロセス上のメモリ状態を想像してみる。クロージャの実体はプロセスのメモリ空間上に（おそらくスタック領域に）展開され、各スレッドはそのアドレスに配置されたクロージャを各々実行する。ここで、_各スレッドのメモリ空間にクロージャのコードセグメントがコピーされるわけではない_。なので、 Sync トレイトが要求しているのはこういうケースで安全に実行できるかどうか、という話...だと思う。

これで合ってるかどうかは自信がないし、どうであればその要件は満たされるのか？という部分に関しても自信持って說明できるほど理解がないので別で勉強する必要がある

（Understanding the Linux Kernel や 初めて読む486 の関連章を読み直さねば）

↑のトレイト境界の話を修正すると、次のようなエラーが出る。

まず、トレイト境界を修正した直後の `rayon::join` を使った実装はこんな感じ

```rust
            // in do_sort()
            rayon::join(
                || do_sort(&mut x[..mid_point], true, comparator),
                || do_sort(&mut x[mid_point..], false, comparator)
```

コンパイルエラーの番号は以下

```plain
$ cargo check
...
error[E0524]: two closures require unique access to `x` at the same time
...
```

これは、2つのクロージャで、同じ変数に対して可変参照を渡しているのが所有権システム的にアウトだと言っている。

これの対策として、子スレッドに渡す前に x を分割しておけばOK、という考え方ができる。これは vecter の `slice_as_mut()` を使えばOK。

### step 5

ベンチマーク実装。手元の Mac で実行してみたらこんな感じだった。すべて n=1 なので性能指標はあくまで参考程度に。

2^25 elements

```plain
$ cargo run --release --example benchmark -- 25
    Finished release [optimized] target(s) in 0.11s
     Running `target/release/examples/benchmark 25`
sorting 33554432 integers (128.0 MB)
cpu info: 4 physical cores, 8 logical cores
seq_sort: sorted 33554432 integers in 17.714332883 seconds
par_sort: sorted 33554432 integers in 4.838593842 seconds
speed up: 3.66x
```

2^26 elements

```plain
$ cargo run --release --example benchmark -- 26

   Compiling rust-sort v0.1.0 (/path/to/project/rust-sort)
    Finished release [optimized] target(s) in 1.00s
     Running `target/release/examples/benchmark 26`
sorting 67108864 integers (256.0 MB)
cpu info: 4 physical cores, 8 logical cores
seq_sort: sorted 67108864 integers in 36.801756354 seconds
par_sort: sorted 67108864 integers in 10.808363674 seconds
speed up: 3.40x
```

2^27 elements

```plain
$ cargo run --release --example benchmark -- 27
    Finished release [optimized] target(s) in 0.11s
     Running `target/release/examples/benchmark 27`
sorting 134217728 integers (512.0 MB)
cpu info: 4 physical cores, 8 logical cores
seq_sort: sorted 134217728 integers in 76.727949211 seconds
par_sort: sorted 134217728 integers in 25.940408823 seconds
speed up: 2.96x
```

2^28 elements

```plain
$ cargo run --release --example benchmark -- 28
    Finished release [optimized] target(s) in 0.03s
     Running `target/release/examples/benchmark 28`
sorting 268435456 integers (1024.0 MB)
cpu info: 4 physical cores, 8 logical cores
seq_sort: sorted 268435456 integers in 162.043424973 seconds
par_sort: sorted 268435456 integers in 51.209880452 seconds
speed up: 3.16x
```

2^29 elements

```plain
$ cargo run --release --example benchmark -- 29
    Finished release [optimized] target(s) in 0.10s
     Running `target/release/examples/benchmark 29`
sorting 536870912 integers (2048.0 MB)
cpu info: 4 physical cores, 8 logical cores
seq_sort: sorted 536870912 integers in 347.513755494 seconds
par_sort: sorted 536870912 integers in 105.172578288 seconds
speed up: 3.30x
```

2^30 elements

```plain
$ cargo run --release --example benchmark -- 30
    Finished release [optimized] target(s) in 0.10s
     Running `target/release/examples/benchmark 30`
sorting 1073741824 integers (4096.0 MB)
cpu info: 4 physical cores, 8 logical cores
seq_sort: sorted 1073741824 integers in 754.570826508 seconds
par_sort: sorted 1073741824 integers in 222.240794225 seconds
speed up: 3.40x
```

Number of elements<br />(2^n count) | sequential sort<br/>duration (sec) | parallel sort<br/>duration (sec)
:--- | :--- | :---
25 | 17.714332883 | 4.838593842
26 | 36.801756354 | 10.808363674
27 | 76.727949211 | 25.940408823
28 | 162.043424973 | 51.209880452
29 | 347.513755494 | 105.172578288
30 | 754.570826508 | 222.240794225
