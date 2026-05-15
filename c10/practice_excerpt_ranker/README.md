# Chapter 10 Practice: Log Triage

Rust Book: https://doc.rust-lang.org/book/ch10-00-generics.html

## 課題

小さなログ監査ツールを作ってください。

アプリケーションには、ログイン失敗、APIエラー、設定変更などのイベントが流れてきます。イベントの種類ごとに持っている情報は違いますが、監査担当者が知りたいのは共通して次の3つです。

- 何が起きたか
- どれくらい危険か
- 先に確認すべきイベントはどれか

`main` にサンプルイベントをいくつか作り、危険度が高いものを優先して表示するプログラムにしてください。

## データ例

最低2種類のイベントを作ってください。

```rust
struct LoginFailure {
    user: String,
    ip_address: String,
    attempts: i32,
    message: String,
}

struct ApiError {
    endpoint: String,
    status_code: i32,
    count: i32,
    message: String,
}
```

サンプルデータ例:

```rust
let login = LoginFailure {
    user: String::from("admin"),
    ip_address: String::from("203.0.113.10"),
    attempts: 8,
    message: String::from("admin user failed login repeatedly from an unfamiliar network"),
};

let api = ApiError {
    endpoint: String::from("/v1/payments"),
    status_code: 500,
    count: 12,
    message: String::from("payment endpoint returned repeated internal server errors"),
};
```

## 実装要件

1. 各イベントに「表示用の説明」と「危険度スコア」を持たせる。
2. `LoginFailure` の危険度は、たとえば `attempts * 10` で計算する。
3. `ApiError` の危険度は、たとえば `count * 5` に、`status_code >= 500` なら `30` を足す。
4. 危険度が `70` 以上なら `HIGH`、`40` 以上なら `MEDIUM`、それ以外なら `LOW` と表示する。
5. `Alert<T>` のようなジェネリック構造体を作り、イベント1件を包む。
6. `Alert<T>` には、説明・危険度・分類を表示するメソッドを実装する。
7. 2つのイベント説明 `&str` を受け取り、長い方を返す関数を作る。
8. 返ってきた説明を、参照を持つ構造体に保存して表示する。

## 出力例

```text
=== Alert ===
login failure: user=admin ip=203.0.113.10 attempts=8
score: 80
severity: HIGH

=== Alert ===
api error: endpoint=/v1/payments status=500 count=12
score: 90
severity: HIGH

longest message:
admin user failed login repeatedly from an unfamiliar network
```

## 制約

- `String` の不要なコピーは避けてください。
- `Alert<T>` の表示処理は、`LoginFailure` と `ApiError` で別々に重複実装しないでください。
- `Vec<Box<dyn Trait>>` は使わないでください。Chapter 18 の範囲なので、今回の必須要件からは外します。

## 考えるポイント

- イベントごとに違う情報から、共通の「説明」と「危険度」をどう取り出すか。
- 危険度の計算ルールを、どこに置くと読みやすいか。
- `Alert<T>` は、どんな条件を満たす `T` に対してだけ表示メソッドを持つべきか。
- 長い方の `&str` を返す関数の戻り値は、どちらの引数に依存しているか。

## 追加課題

- `ConfigChange` イベントを追加する。
- 複数の `LoginFailure` を `Vec<Alert<LoginFailure>>` に入れ、危険度順に並べる。
- 同じIPアドレスからのログイン失敗回数を集計する。
- `impl Trait` を返す関数で、デフォルトのテストイベントを1件作る。

## コマンド

```bash
cargo run -p practice_excerpt_ranker
cargo fmt -- --check
cargo clippy -p practice_excerpt_ranker
```

`cargo clippy` は任意です。
