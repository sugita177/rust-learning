# 04_structs: 構造体とメソッド（Structs & Methods）

複数の関連するデータを意味のある1つの型としてまとめる「構造体（`struct`）」の基本、フィールド初期化省略記法、構造体更新記法（`..`）と所有権の部分ムーブ、デバッグ出力（`#[derive(Debug)]`）、およびタプル構造体について学ぶ演習です。

---

## 実行コマンド

```bash
# 構造体演習バイナリの実行
cargo run -p structs

# コンパイルチェック
cargo check -p structs
```

---

## 1. 基本事項・概念の整理

### 1.1 構造体（`struct`）の定義とインスタンス化
構造体は、複数の関連する値を名前付きフィールドとしてひとまとめにするカスタムデータ型です。

- **定義**:
  ```rust
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }
  ```
- **可変性のルール**:
  - Rust では「特定のフィールドだけを `mut` にする」ことはできません。
  - フィールドを書き換えたい場合は、インスタンス全体を `let mut user = ...` として宣言します。

### 1.2 フィールド初期化省略記法（Field Init Shorthand）
関数の引数名やローカル変数名が構造体のフィールド名と全く同じ場合、`username: username` と繰り返さず、`username` とだけ書くことができます。

```rust
fn build_user(username: String, email: String) -> User {
    User {
        username, // username: username の省略
        email,    // email: email の省略
        sign_in_count: 1,
        active: true,
    }
}
```

### 1.3 構造体更新記法（`..`）と部分ムーブ（Partial Move）
既存のインスタンスの値を引き継いで新しいインスタンスを作成する際、`..既存のインスタンス` 構文が使えます。

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1 // username, sign_in_count, active を user1 から引き継ぐ
};
```
- **代入時の挙動（ムーブとコピーの混在）**:
  - `String` などの非 `Copy` 型フィールドは、元のインスタンスから新しいインスタンスへ **所有権がムーブ** します。
  - `u64` や `bool` などの `Copy` 型フィールドは、値が **スタック上でコピー** されます。
- **部分ムーブ（Partial Move）**:
  - 一部のフィールドだけが所有権を失い、残りのフィールドは所有権を保持している状態。
  - この状態になった元のインスタンスは、**全体としては使用不可**（未完全な状態）になります。

### 1.4 デバッグ出力（`#[derive(Debug)]`）
通常、自作の構造体は `println!("{}", user)` では出力できません（`Display` トレイトが未実装のため）。
構造体の定義の上に **`#[derive(Debug)]`** を付与することで、`Debug` トレイトが自動実装され、中身を簡単に確認できるようになります。
- `{:?}` : 1 行でコンパクトに出力
- `{:#?}`: 改行・インデント付きで見やすく整形して出力

### 1.5 タプル構造体（Tuple Structs）と型安全性
フィールドに名前をつけず、型の並びだけで定義する構造体です。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```
- 各要素にはタプルと同様に `c.0`, `c.1`, `c.2` でアクセスします。
- **単なるタプル `(i32, i32, i32)` との決定的な違い**:
  - `Color` と `Point` は、保持するデータ型と要素数が同一であっても、**コンパイル時に厳密に別の型として区別** されます。
  - 「`Point` を受け取る関数に誤って `Color` を渡してしまう」といったバグをコンパイル時に 100% 防ぐことができます（Newtype パターンの基礎）。

---

## 2. 学び・検証記録（エラー体験と挙動実証）

### 2.1 [コンパイルエラー] 構造体更新記法における部分ムーブと全体アクセス禁止（E0382）
- **コード**:
  ```rust
  let user1 = build_user(String::from("user"), String::from("user@example.com"));
  let user2 = User {
      email: String::from("another@example.com"),
      ..user1 // user1.username が user2 へムーブする
  };

  // user1.username にアクセスしようとする
  println!("username: {}", user1.username);
  
  // user1 全体を出力しようとする
  println!("user1 full = {:?}", user1);
  ```
- **発生したエラー**:
  ```text
  error[E0382]: borrow of moved value: `user1.username`
    --> exercises/04_structs/src/main.rs:40:30
     |
  29 |       let user2 = User {
     |  _________________-
  30 | |         email: String::from("another@example.com"),
  31 | |         ..user1
  32 | |     };
     | |_____- value moved here
  ...
  40 |       println!("username: {}", user1.username);
     |                                ^^^^^^^^^^^^^^ value borrowed here after move
     |
     = note: move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait
  ```
- **学び**:
  - `user1.username` は `user2` に所有権が移動したためアクセス不可。
  - また、一部のパーツが抜けた状態の `user1` は不完全とみなされ、`println!("{:?}", user1)` のように構造体丸ごとのアクセスも完全に遮断される。

### 2.2 [検証・考察] 部分ムーブ後の構造体の使い道とメモリ・Drop の挙動
- **疑問**: 一部の所有権がなくなった構造体は、メモリ上に残しておいてどのような使い道があるのか？
- **実務での用途**:
  1. **分解（Destructuring）してパーツを個別利用**:
     - 重いデータ（`String` 等）だけを別スレッドや関数にムーブし、残った軽量なステータスや ID を手元で利用・ログ出力する。
  2. **再代入による完全体の復活**:
     - `user1` が `mut` であれば、抜けた `username` に新しい `String` を再代入（`user1.username = ...`）することで、再び完全なインスタンスとして全体を利用できるようになる。
- **メモリと Drop の賢い挙動**:
  - `user1` のスタック領域は関数終了まで物理的に残るが、**Drop（メモリ解放）されるのは「まだ所有権が残っているフィールド（`user1.email`）」のみ**。
  - ムーブ済みの `user1.username` の Drop はスキップされ、`user2` 側で 1 度だけ Drop されるため、二重解放は絶対に起きない。

### 2.3 [コンパイルエラー] タプル構造体による型の取り違え防止（E0308）
- **コード**:
  ```rust
  fn show_point(p: &Point) { ... }

  let color1 = Color(255, 0, 0);
  show_point(&color1); // Color 型を Point 型を受け取る関数に渡そうとする
  ```
- **発生したエラー**:
  ```text
  error[E0308]: mismatched types
    --> exercises/04_structs/src/main.rs:90:16
     |
  90 |     show_point(&color1);
     |     ---------- ^^^^^^^ expected `&Point`, found `&Color`
  ```
- **実証できたこと**:
  - どちらも内部的には `(i32, i32, i32)` という全く同じデータ表現であっても、型名が異なればコンパイラが完全に別の型として扱い、混同をコンパイル時に阻止してくれる。
