# Rust Book 学習リポジトリ

[Rust Book](https://doc.rust-lang.org/book/title-page.html) を使用した Rust の学習を行う。

## 構成

章ごとに `c1/`, `c2/` のようなディレクトリを切り、その下に Cargo project を置く。

例:

```text
c1/hello_cargo
c2/guessing_game
c6/match_expression
```

このリポジトリは Cargo workspace として管理している。

```bash
cargo check
cargo run -p match_expression
```
