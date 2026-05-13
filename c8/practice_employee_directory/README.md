# Chapter 8 Practice: Employee Directory

Rust Book: https://doc.rust-lang.org/book/ch08-00-common-collections.html

## 目的

Chapter 8 の summary にある「HashMap と vector を使って、部署ごとの社員管理をするテキストインターフェイス」を作ります。

`Vec`、`String`、`HashMap` を組み合わせ、`entry().or_insert(...)` で「キーがなければ初期値を入れる」処理を練習します。

## 課題

会社の部署と社員名を管理する小さなコマンドラインプログラムを作ってください。

ユーザーが次のようなコマンドを入力できるようにします。

```text
Add Sally to Engineering
Add Amir to Sales
List Engineering
List All
Quit
```

部署ごとの社員一覧、または会社全体の社員一覧を表示できるようにしてください。

## 要件

1. 社員データは `HashMap<String, Vec<String>>` で管理する。
2. `Add <name> to <department>` 形式の入力で、指定された部署に社員名を追加する。
3. 部署がまだ存在しない場合は、`entry().or_insert(...)` または `or_insert_with(...)` を使って空の `Vec<String>` を作る。
4. 同じ部署に同じ社員名を重複登録しない。
5. `List <department>` で、その部署の社員名をアルファベット順に表示する。
6. 存在しない部署を指定された場合は、見つからなかったことを表示する。
7. `List All` で、部署名をアルファベット順に並べ、各部署の社員名もアルファベット順に表示する。
8. `Quit` が入力されたら終了する。

## 動作確認例

入力例:

```text
Add Sally to Engineering
Add Amir to Sales
Add Ana to Engineering
Add Sally to Engineering
List Engineering
List All
Quit
```

出力例:

```text
Engineering:
- Ana
- Sally

Engineering:
- Ana
- Sally

Sales:
- Amir
```

`Add Sally to Engineering` が 2 回入力されても、`Sally` は 1 回だけ表示されるようにします。

## 実装のヒント

まずは文字列を `split_whitespace()` で分割して構いません。

```rust
let parts: Vec<&str> = input.split_whitespace().collect();
```

社員追加では、次のような考え方を使います。

```rust
let employees = company.entry(department).or_insert(Vec::new());
```

または:

```rust
let employees = company.entry(department).or_insert_with(Vec::new);
```

`employees` は `Vec<String>` への `&mut` なので、重複チェック後に `push` できます。

## 確認ポイント

- `Vec<T>` に複数の値を保存できる
- `String` と `&str` の使い分けを考えられる
- `HashMap<K, V>` でキーから値を取り出せる
- `entry().or_insert(...)` が返す `&mut V` を使って値を更新できる
- `HashMap` の走査順は保証されないため、表示前にソートする必要がある
- 所有権を HashMap に移す場面と、一時的に参照する場面を区別できる

## 追加課題

- `Remove <name> from <department>` を追加する。
- 部署名や社員名の大文字小文字をどう扱うか決めて実装する。
- `Add Sally to Engineering` 以外の不正な入力に対して、わかりやすいエラーメッセージを表示する。
- 社員名の一覧表示を関数に切り出す。

## コマンド

```bash
cargo run -p practice_employee_directory
cargo fmt -- --check
cargo clippy -p practice_employee_directory
```

`cargo clippy` は任意です。まだ Clippy を使っていない場合は、`cargo fmt -- --check` までで構いません。
