# mini_parser: 数式パーサ＆AST走査エンジン

数式の字句解析（Lexer）、構文解析（Parser）、抽象構文木（AST）、および構文木の評価・走査（Visitor）を一貫して実装するミニプロジェクトです。

---

## 実行コマンド

```bash
# バイナリの実行
cargo run -p mini_parser

# 単体テストの実行
cargo test -p mini_parser

# コンパイルチェック
cargo check -p mini_parser
```

---

## 開発ステップ

- [x] **01. 木構造とAST定義 (`mini-parser/01-tree-structure`)**
- [ ] **02. 字句解析器（Lexer） (`mini-parser/02-lexer`)**
- [ ] **03. 構文解析器（Parser） (`mini-parser/03-parser`)**
- [ ] **04. 構文木走査（Visitor） (`mini-parser/04-visitor`)**

---

## 01. 木構造とAST定義の記録

### 1.1 基本概念
- **抽象構文木（AST: Abstract Syntax Tree）**:
  - ソースコードや数式の文法構造を、階層的な木構造として表現したデータ構造。
- **再帰的データ構造と `Box<T>`**:
  - 木構造は「式の中にさらに式が含まれる」再帰的な構造を持つ。
  - Rust ではコンパイル時にスタックサイズが確定（Sized）していなければならないため、直接入れ子にするとサイズが無限大となりコンパイルエラーとなる。
  - 実体データをヒープ領域に確保し、スタック上には固定長（64bit環境で8バイト）のポインタを持つ `Box<T>` を挟むことでサイズを確定させる。
- **`PartialEq` と `Eq`**:
  - `PartialEq`: `==` による等値比較を可能にするトレイト。テストマクロ `assert_eq!` に必要。
  - `Eq`: 「`a == a` が常に true である（反射律）」ことを保証するトレイト。浮動小数点数（`f64` の `NaN`）にはない厳密な同一性を保証する。

---

### 1.2 学び・検証記録

#### [エラー実証] `E0072`: recursive type has infinite size
`Box` を使わずに `Expr` を直接再帰定義した場合のコンパイルエラー。

- **検証コード**:
  ```rust
  pub enum Expr {
      Number(i64),
      Add(Expr, Expr), // コンパイルエラー！
      Multiply(Expr, Expr),
  }
  ```
- **コンパイラ出力**:
  ```text
  error[E0072]: recursive type `Expr` has infinite size
   --> projects/mini_parser/src/ast.rs:3:1
    |
  3 | pub enum Expr {
    | ^^^^^^^^^^^^^
  ...
  8 |     Add(Expr, Expr),
    |         ---- recursive without indirection
    |
  help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
    |
  8 |     Add(Box<Expr>, Expr),
    |         ++++    +
  ```
- **解消法**:
  - `Box<Expr>` を用いて間接参照（Indirection）を導入し、ポインタサイズ（8バイト）に固定化。

---

#### [正常系実証] 再帰的評価（`eval`）と単体テスト
- **AST の定義と実装**:
  ```rust
  #[derive(Debug, PartialEq, Eq)]
  pub enum Expr {
      Number(i64),
      Add(Box<Expr>, Box<Expr>),
      Multiply(Box<Expr>, Box<Expr>),
  }

  impl Expr {
      pub fn eval(&self) -> i64 {
          match self {
              Expr::Number(n) => *n,
              Expr::Add(left, right) => left.eval() + right.eval(),
              Expr::Multiply(left, right) => left.eval() * right.eval(),
          }
      }
  }
  ```
- **テスト実行結果**:
  ```text
  running 3 tests
  test ast::tests::test_eval_simple_number ... ok
  test ast::tests::test_eval_add_and_multiply ... ok
  test ast::tests::test_eval_nested_tree ... ok

  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
  ```
- **実証できたこと**:
  - ヒープメモリ上に木構造（`(1 + 2) * 3` 等）を安全に構築でき、パターンマッチによる再帰走査で正しく計算（評価）できることを確認。
