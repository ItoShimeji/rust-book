# Chapter 6 Practice: Enums and Pattern Matching

Rust Book: https://doc.rust-lang.org/book/ch06-00-enums.html

## 目的

Chapter 6 で扱う enum、データを持つバリアント、`Option`、`match`、`if let` を使って、状態や入力の種類によって処理を分ける設計を練習します。

## 課題

小さな「イベントルーター」を作ってください。アプリに届いたイベントを enum で表し、イベントの種類ごとにメッセージを生成したり、必要なら内部状態を更新したりします。

## 要件

1. `AppEvent` enum を定義し、少なくとも次のバリアントを持たせること。
   - `Login { user: String }`
   - `Message { from: String, text: String }`
   - `Warning(String)`
   - `Logout`
   - `Noop`
2. `RouterState` struct を定義し、現在ログイン中のユーザーを `Option<String>` で保持すること。
3. `RouterState::new()` を実装し、ログイン中ユーザーがいない初期状態を作ること。
4. `handle_event(state: &mut RouterState, event: AppEvent) -> String` を実装すること。
5. `handle_event` では `match` を使い、すべての `AppEvent` バリアントを明示的に処理すること。
6. `Login` を受け取ったら `state` のログイン中ユーザーを更新し、`"<user> logged in"` のような文字列を返すこと。
7. `Message` を受け取ったら、本文が空文字なら `"empty message from <from>"`、それ以外なら `"<from>: <text>"` のような文字列を返すこと。
8. `Logout` を受け取ったら、`if let` を使ってログイン中ユーザーがいる場合だけ名前を取り出し、状態を未ログインに戻すこと。
9. `Warning` と `Noop` もそれぞれ意味のある文字列を返すこと。
10. `main` では複数のイベントを順番に処理し、返された文字列を `println!` で表示すること。

## 動作確認例

たとえば次のようなイベント列を用意します。

```rust
Login { user: "Aki".to_string() }
Message { from: "Aki".to_string(), text: "hello".to_string() }
Warning("disk almost full".to_string())
Logout
Noop
```

出力は完全一致でなくても構いませんが、各イベントが別々の分岐で処理され、ログイン状態の更新と解除が確認できる内容にしてください。

## 確認ポイント

- enum で「取りうる状態や入力の種類」を表現できているか
- バリアントに関連データを持たせられているか
- `match` ですべてのバリアントを漏れなく処理できているか
- `Option<String>` で値がある場合とない場合を表現できているか
- `if let` を使って、特定のパターンだけを簡潔に扱えているか

## 実行コマンド

```bash
cargo run -p practice_event_router
cargo fmt -- --check
cargo clippy -p practice_event_router
```

`cargo clippy` は任意ですが、余裕があれば警告が出ない状態を目指してください。

## 追加課題

- `AppEvent::Command { name: String, args: Vec<String> }` を追加し、コマンド名によって処理を分ける。
- `Message` の本文に `"urgent"` が含まれている場合だけ、別の警告メッセージを返す。
- `handle_event` の挙動を確認する単体テストを追加する。
