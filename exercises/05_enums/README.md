# 05_enums: Enumとパターンマッチ（Enums & Pattern Matching）

Rust の最も表現力豊かな機能の1つである「列挙型（`enum`）」、各ヴァリアントへのデータ埋め込み、`match` 式による網羅的なパターンマッチ、`null` のない安全な世界を実現する `Option<T>`、および `if let` 構文について学ぶ演習です。

---

## 実行コマンド

```bash
# Enum演習バイナリの実行
cargo run -p enums

# コンパイルチェック
cargo check -p enums
```

---

## 1. 基本事項・概念の整理

### 1.1 他言語との決定的な違い：「値を持てる Enum」
C/C++、Java、TypeScript などの Enum は基本的に「0, 1, 2... という名前付き整数」ですが、Rust の Enum は **代数的データ型（Tagged Union）** です。
各ヴァリアント（選択肢）ごとに、**全く異なる型や構造のデータを直接埋め込む** ことができます。

```rust
#[derive(Debug)]
enum Message {
    Quit,                       // データなし（従来のEnum相当）
    Move { x: i32, y: i32 },    // 構造体のような名前付きフィールド
    Write(String),              // 単一の String
    ChangeColor(i32, i32, i32), // 3つの整数のタプル
}
```

- **メリット**:
  - 構造体だけで表現しようとすると、`QuitMessage`、`MoveMessage` など個別の構造体を多数定義しなければならず、共通して受け取る関数が作りにくい。
  - Enum を使えば、これら異なる形状のデータをすべて **「`Message` 型」という単一の型** として変数や関数の引数で統一的に扱える。

### 1.2 Enum のメソッド定義（`impl`）
構造体と同様に、Enum に対しても `impl Message { ... }` でメソッドを定義できます。
自身を表す `&self` を受け取り、内部のヴァリアントに応じた処理を呼び出すことができます。

### 1.3 `match` 式によるパターンマッチと値の取り出し（パターンバインディング）
Enum の中に埋め込まれたデータを取り出す基本手段は **`match` 式** です。
ヴァリアントの判定と同時に、内部のデータを変数に束縛（バインド）して取り出せます。

```rust
match self {
    Message::Quit => println!("終了"),
    Message::Move { x, y } => println!("({}, {})へ移動", x, y),
    Message::Write(text) => println!("{}", text),
    Message::ChangeColor(r, g, b) => println!("({}, {}, {})へ変更", r, g, b),
}
```

### 1.4 `match` の鉄則：網羅性（Exhaustiveness）とワイルドカード（`_`）
- **網羅性チェック**:
  - Rust の `match` は、**すべての可能性（ヴァリアント）を漏れなく記述する** ことが義務付けられています。
  - 1 つでも漏れがあるとコンパイルエラー（`E0004`）となり、分岐の処理漏れバグが言語仕様レベルで 100% 根絶されます。
- **ワイルドカード（`_`）**:
  - 特定のパターン以外をすべてまとめて処理したい場合、末尾に `_ => { ... }` を置くことで「その他すべて」を表現できます。

### 1.5 `Option<T>` による「null のない世界」
多くの言語に存在する `null`（`nil` / `undefined`）は、存在しない値を参照してクラッシュするバグの最大の温床となってきました。

**Rust には `null` が存在しません。**
その代わり、標準ライブラリで以下の Enum が最初から定義されています（`std::option::Option` は自動インポートされているため、`Some` や `None` とそのまま書けます）。

```rust
enum Option<T> {
    None,    // 値が存在しない（null相当）
    Some(T), // 型 T の有効な値が存在する
}
```

- **なぜ安全なのか？（型システムによる保証）**:
  - `Option<i32>` と `i32` は **全く別の型** です。
  - そのため、`Some` の中身を安全に取り出さない限り、通常の数値演算（足し算など）に使うことができません。
  - 「うっかり null かもしれない値を通常の値として使ってしまう」ミスがコンパイル時に完全に遮断されます。

### 1.6 `if let` 構文による簡潔な制御フロー
「ある 1 つのパターン（例えば `Some`）のときだけ処理を行い、それ以外（`None`）は無視したい」という場合、`match` で `_ => ()` と書くのは冗長です。
そんなときは **`if let`** を使うことで簡潔に記述できます。

```rust
if let Some(x) = six {
    println!("{}", x);
}
```
- `if let パターン = 式 { ... }` の形式で、パターンにマッチしたときだけブロックを実行する糖衣構文（シンタックスシュガー）です。`else` を付けることも可能です。

---

## 2. 学び・検証記録（エラー体験と挙動実証）

### 2.1 [コンパイラ警告] 未読フィールドの警告と `match` による解消
- **発生した警告**:
  ```text
  warning: fields `x` and `y` are never read
  warning: field `0` is never read
  warning: fields `0`, `1`, and `2` are never read
  ```
- **学び**:
  - `println!("{:?}", self)`（Debug トレイト）で出力するだけでは、コンパイラは「中のフィールドを読み取った」とみなさない。
  - 各ヴァリアントの内部データを `match` 式で取り出して使用したところ、この警告は完全に解消された。

### 2.2 [コンパイルエラー] パターンマッチの非網羅性エラー（E0004）
- **コード**:
  ```rust
  match self {
      Message::Quit => { ... }
      Message::Move { x, y } => { ... }
      Message::Write(text) => { ... }
      // Message::ChangeColor を意図的にコメントアウト
  }
  ```
- **発生したエラー**:
  ```text
  error[E0004]: non-exhaustive patterns: `&Message::ChangeColor(_, _, _)` not covered
    --> exercises/05_enums/src/main.rs:15:15
     |
  15 |         match self {
     |               ^^^^ pattern `&Message::ChangeColor(_, _, _)` not covered
     |
  note: `Message` defined here
    --> exercises/05_enums/src/main.rs:6:5
     |
   6 |     ChangeColor(i32, i32, i32),
     |     ----------- not covered
  ```
- **なぜコンパイラが防ぐのか（何が危険なのか）**:
  - もしこれを許可すると、実行時に `ChangeColor` が渡された際、どの分岐にも入れず未定義動作やクラッシュを引き起こす。
  - C言語等の `switch` 文にありがちな「`case` の追加忘れ・分岐漏れ」を、コンパイルエラーとして 100% 阻止してくれる。
  - 将来 Enum に新しいバリエーションを追加した際にも、修正が必要なすべての `match` 箇所をコンパイラが教えてくれるため、大規模リファクタリングが極めて安全に行える。

### 2.3 [挙動検証] ワイルドカード（`_`）によるデフォルト分岐の実証
- **検証コード**:
  ```rust
  fn call_partial(&self) {
      match self {
          Message::Quit => println!("終了"),
          _ => println!("終了以外のメッセージ"),
      }
  }
  ```
- **実行結果**:
  ```text
  終了
  終了以外のメッセージ
  終了以外のメッセージ
  終了以外のメッセージ
  ```
- **実証できたこと**:
  - `m1`（`Quit`）のみが第1アームにマッチし、残りの `m2, m3, m4`（`Move`, `Write`, `ChangeColor`）はすべて `_` に吸収された。
  - 特定の関心のあるパターン以外を安全かつ簡潔にひとまとめにできることを確認。

### 2.4 [挙動検証] `Option<T>` と `if let` による安全な値のハンドリング
- **検証コード**:
  ```rust
  fn plus_one(x: Option<i32>) -> Option<i32> {
      match x {
          None => None,
          Some(i) => Some(i + 1),
      }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  if let Some(x) = six {
      println!("{}", x);
  }
  if let None = none {
      println!("None");
  }
  ```
- **実行結果**:
  ```text
  six = Some(6)
  none = None
  6
  None
  ```
- **実証できたこと**:
  - `Some(5)` に対しては中身の `5` を取り出して `Some(6)` を返し、`None` に対しては何もせず `None` を返す安全な条件分岐が実現できた。
  - `if let` 構文により、`Some` だけでなく `None` に対しても単一パターンのマッチが直感的に行えることが実証された。
