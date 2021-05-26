# README

出典: [実践Rustプログラミング入門](https://www.amazon.co.jp/dp/B08PF27TRZ)

Bitonic-sort の実装


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

