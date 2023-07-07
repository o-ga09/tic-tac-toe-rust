# Rust 学んだことメモ

- lib.rsに書くモジュール名は、srcディレクトリのファイル名と一致させる
- 新しくパッケージを作成すr時は、cargo new パッケージ名 で作成する
- 新しく作成したパッケージは、ルートディレクトリのCargo.tomlに追加する
  - 追加する対象
    - [dependencies]、[workspace]

~~~toml
[package]
name = "tic-tac-toe-rust"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
domain = { path = "./domain" }    # 追加
usecase = { path = "./usecase" }　# 追加

[workspace]
members = [
    "domain",　　# 追加
    "usecase"　　# 追加
]

~~~


## 抽象化クラスについて

- traitを使用する
- 実装は以下

~~~rust

// 定義
use domain::domain::Board;
use domain::domain::Koma;

pub trait Port {
    fn iswin(board: Board) -> bool;
    fn display(board: Board);
    fn input(player: i32) -> Koma;
}

// 実装


~~~