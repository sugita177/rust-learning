# 09_modules_visibility: モジュールシステムと可視性（Modules & Visibility）

Rust におけるコードの構造化、モジュールツリーの構築、ファイル分割（`mod`）、名前空間のインポート（`use`）、およびカプセル化を支える可視性制御（`pub`, `pub(crate)`, private）について学ぶ演習です。

---

## 実行コマンド

```bash
# モジュールと可視性演習バイナリの実行
cargo run -p modules_visibility

# コンパイルチェック
cargo check -p modules_visibility
```

---

## 1. 基本事項・概念の整理

### 1.1 Rust のモジュールシステムと他言語との違い
- **他言語（TypeScript / Python 等）**:
  - ファイルを作成すれば自動的にモジュールとなり、`import` 側でパスを指定して呼び出す。
- **Rust の原則**:
  - **ファイルが存在するだけではコンパイル対象にならない**。
  - 親モジュール（バイナリのルートである `main.rs` やライブラリのルートである `lib.rs`）で明示的に **`mod ファイル名;`** と宣言して初めて、ルートを頂点とする「モジュールツリー（木構造）」に組み込まれる。

```text
crate (main.rs)
 ├── config (インラインモジュール)
 └── math (math.rs)
      └── advanced (サブモジュール)
```

### 1.2 可視性（Visibility）の基本原則
- **デフォルトは完全非公開（Private）**:
  - 関数、構造体、列挙型、フィールド、メソッドなど、何も修飾子をつけなければ同一モジュール内（またはその子モジュール）からしか見えません。
- **親子間のアクセス規則**:
  - **親から子へ**: 子モジュールの非公開アイテムにはアクセスできない（明示的な `pub` が必要）。
  - **子から親へ**: 子モジュールからは親モジュールのアイテム（非公開含む）を常に直接参照できる（`super::` 等）。

### 1.3 構造体とフィールドの可視性の罠
- **構造体自体を `pub` にしても、内部のフィールドは自動的に `pub` にはならない**（デフォルトで private）。
- **カプセル化の保護**:
  - 構造体に非公開フィールドが1つでもある場合、外部モジュールから構造体リテラル構文（`Struct { field: val }`）で直接インスタンス化することはコンパイルエラー（`E0451`）となります。
  - 外部にインスタンス化を許可したい場合、**`pub fn new(...) -> Self`** などの公開コンストラクタ関数を提供し、値の読み取りには公開ゲッターメソッドを公開するのが Rust の鉄則です。

### 1.4 パスの参照と `use`
- **絶対パス（`crate::...`）**: クレートのルート（`main.rs` や `lib.rs`）から順に指定するパス。
- **相対パス**:
  - `self::...`: 現在のモジュールを起点とする。
  - `super::...`: 1つ上の親モジュールを起点とする（相対パスでの親参照）。
- **`use` キーワード**:
  - 長いモジュールパスを現在のスコープにインポートし、短い名前でアクセスできるようにする。

### 1.5 クレート境界と `pub(crate)`

| 修飾子 | 公開範囲 | 主な用途 |
| :--- | :--- | :--- |
| **（なし / private）** | **現在のモジュール（およびその子モジュール）のみ** | モジュール内部の完全な実装隠蔽 |
| **`pub(crate)`** | **現在のクレート（プロジェクト全体）** | クレート内（複数モジュール間）で共有するが、ライブラリの外部利用者には隠したい内部API |
| **`pub`** | **全世界（外部クレート含むすべて）** | 一般に公開する正式なパブリックAPI |

#### パッケージとクレート境界の成立パターン
1. **外部依存パッケージ**: `Cargo.toml` に指定したサードパーティ製ライブラリ。内部の `pub(crate)` は外部から一切見えない。
2. **`lib.rs` と `main.rs`**: 同一パッケージ内であっても、ライブラリ（`lib.rs`）とバイナリ（`main.rs`）は **別々のクレート** として扱われる。そのため `lib.rs` 内の `pub(crate)` は `main.rs` からはアクセスできない。
3. **統合テスト（`tests/`）**: テストファイルも独立したクレートとしてコンパイルされるため、`pub` な API のみがテスト可能。

---

## 2. 学び・検証記録（コンパイルエラーと挙動実証）

### 2.1 [コンパイルエラー実証] `E0603`: function is private
インラインモジュール内の非公開関数を親モジュールから直接呼び出そうとした場合のエラー。

- **検証コード**:
  ```rust
  mod config {
      fn get_port() -> u16 { 8080 } // private
  }

  fn main() {
      let port = config::get_port(); // コンパイルエラー！
  }
  ```
- **コンパイラ出力**:
  ```text
  error[E0603]: function `get_port` is private
    --> exercises/09_modules_visibility/src/main.rs:12:24
     |
  12 |     let port = config::get_port();
     |                        ^^^^^^^^ private function
  ```
- **解消法**:
  - `pub fn get_port() -> u16` と明示的に公開することでアクセス可能に修正。

---

### 2.2 [コンパイルエラー実証] `E0451`: field of struct is private
構造体自体は公開（`pub`）したが、フィールドが非公開のまま外部から直接インスタンス化しようとした場合のエラー。

- **検証コード**:
  ```rust
  mod config {
      pub struct ServerConfig {
          pub host: String,
          port: u16, // private
      }
  }

  fn main() {
      let config = config::ServerConfig {
          host: "localhost".to_string(),
          port: 8080, // コンパイルエラー！
      };
  }
  ```
- **コンパイラ出力**:
  ```text
  error[E0451]: field `port` of struct `ServerConfig` is private
    --> exercises/09_modules_visibility/src/main.rs:27:9
     |
  25 |     let config = config::ServerConfig {
     |                  -------------------- in this type
  26 |         host: "localhost".to_string(),
  27 |         port: 8080,
     |         ^^^^ private field
  ```
- **解消法**:
  - `impl ServerConfig` 内に `pub fn new(host: String, port: u16) -> Self` コンストラクタ関数と、`pub fn port(&self) -> u16` ゲッターを定義して解決。

---

### 2.3 [正常系実証] ファイル分割・モジュールツリーと `pub(crate)` の動作
別ファイル（`math.rs`）のモジュール化、サブモジュールからの `super` 参照、および `pub(crate)` の呼び出し結果。

- **実行結果**:
  ```text
  ---- 1. インラインモジュールと可視性 ----
  Port: 8080

  ---- 2. 構造体と可視性 ----
  ServerConfig: ServerConfig { host: "localhost", port: 8080 }
  config.host（直接アクセス）: localhost
  config.port（メソッド経由アクセス）: 8080

  ---- 3. ファイル分割とモジュールパス ----
  Sum: 3
  Product: 12
  Add & Doubule: 20

  ---- 4. クレート内部専用関数 ----
  Inner calc: 6
  ```
- **実証できたこと**:
  - `main.rs` での `mod math;` 宣言により、外部ファイル `math.rs` が正常にモジュールツリーに統合された。
  - `math.rs` 内のサブモジュール `advanced` から `super::add` により親モジュールの要素を安全に呼び出せた。
  - `pub(crate) fn inner_calc` が、同一バイナリクレート内である `main.rs` から正しく呼び出せることを確認。
