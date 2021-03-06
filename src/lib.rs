pub mod first;
pub mod second;
pub mod third;
pub mod fourth;
pub mod utils;
pub mod thread_example;

/// bool 型に変わる、ソート順序の指定引数
/// モジュール本体を first, second とステップごとにファイルを分けており、かつこの enum はすべてのモジュールから使いたいのでここで宣言する
pub enum SortOrder {
    /// 昇順
    Ascending,
    /// 降順
    Descending,
}