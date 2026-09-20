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
- [x] **02. 字句解析器（Lexer） (`mini-parser/02-lexer`)**
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

---

## 02. 字句解析器（Lexer）の記録

### 2.1 基本概念
- **字句解析（Lexical Analysis / Tokenization）**:
  - ソースコードの生の文字列を走査し、空白などの不要な情報を捨てながら、文法的な最小構成要素である「トークン（`Token`）」の列に変換する処理。
  - 後続の構文解析器（Parser）の入力となる。
- **`chars().peekable()` によるイテレータの先読み走査**:
  - `input.chars()`: 文字列を 1 文字（`char`）単位でイテレート（スペースや改行もそのまま取得）。
  - `.peekable()`: イテレータのカーソルを進めずに「次の文字を覗き見（peek）」する能力を追加。
  - **`peek()` と `next()` の協調**:
    - `peek()` で次の文字が数字や空白かを確認。
    - 条件に合致していれば `next()` で文字を消費して進める。
    - 複数桁の数値（例: `"123"`）を、非数字が来るまで先読みしながら安全に 1 つの `Token::Number(123)` に集約可能。

### 2.2 学び・検証記録

#### [実装] トークン定義と字句解析関数
- **トークンとエラーの定義**:
  ```rust
  #[derive(Debug, PartialEq, Eq, Clone)]
  pub enum Token {
      Number(i64),
      Plus, Minus, Asterisk, Slash,
      LParen, RParen,
  }

  #[derive(Debug, PartialEq, Eq)]
  pub enum LexError {
      UnexpectedChar(char),
  }
  ```

#### [正常系・異常系実証] 単体テストによる検証
- **検証コード**:
  ```rust
  #[test]
  fn test_tokenize_valid_input() {
      let input = " 1 + ( 23 * 456 ) / 7 - 89 ";
      let tokens = tokenize(input).unwrap();
      assert_eq!(
          tokens,
          vec![
              Token::Number(1), Token::Plus, Token::LParen, Token::Number(23),
              Token::Asterisk, Token::Number(456), Token::RParen,
              Token::Slash, Token::Number(7), Token::Minus, Token::Number(89),
          ]
      );
  }

  #[test]
  fn test_tokenize_invalid_char() {
      let input = "10 + @ - 5";
      assert_eq!(tokenize(input), Err(LexError::UnexpectedChar('@')));
  }
  ```
- **テスト実行結果**:
  ```text
  running 5 tests
  test ast::tests::test_eval_add_and_multiply ... ok
  test ast::tests::test_eval_nested_tree ... ok
  test ast::tests::test_eval_simple_number ... ok
  test lexer::tests::test_tokenize_valid_input ... ok
  test lexer::tests::test_tokenize_invalid_char ... ok

  test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
  ```
- **実証できたこと**:
  - 不規則な空白が綺麗に除去され、四則演算記号、括弧、複数桁の数値が正しくトークン化された。
  - 未知の不正な文字（`@`）が `LexError::UnexpectedChar` として安全に検出されることを確認。
