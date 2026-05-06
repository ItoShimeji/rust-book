# Chapter 5 Practice: Book Logger

Rust Book Chapter 5 のまとめ課題。

## 課題

読書記録を表す `Book` struct を作る。

### フィールド

```rust
title: String
author: String
pages: u32
current_page: u32
```

### 実装するメソッド

```rust
fn new(title: String, author: String, pages: u32) -> Book
```

`Book` を作る関連関数。`current_page` は `0` から始める。

```rust
fn summary(&self) -> String
```

本の概要を文字列で返す。

例:

```text
「The Rust Programming Language」 by Steve Klabnik and Carol Nichols, 500 pages
```

```rust
fn progress(&self) -> f64
```

読書進捗をパーセントで返す。

```rust
fn read(&mut self, pages: u32)
```

読んだページ数だけ `current_page` を進める。
ただし、`current_page` が総ページ数 `pages` を超えないようにする。

```rust
fn is_finished(&self) -> bool
```

読み終わっていれば `true`、そうでなければ `false` を返す。

## 確認ポイント

- `struct` による関連データの整理
- `impl` ブロックによるデータと振る舞いの整理
- 読むだけのメソッドでは `&self` を使う
- インスタンスを変更するメソッドでは `&mut self` を使う
- インスタンス生成用の関連関数では `Self::new` / `Book::new` の形を使う
- `current_page` が `pages` を超えないように境界条件を扱う

## レビューでの注意点

`read` では、加算後のページ数を総ページ数と比較する。

```rust
fn read(&mut self, pages: u32) {
    self.current_page = (self.current_page + pages).min(self.pages);
}
```

より安全にするなら、`u32` の加算オーバーフローも避ける。

```rust
fn read(&mut self, pages: u32) {
    self.current_page = self.current_page.saturating_add(pages).min(self.pages);
}
```
