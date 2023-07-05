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
