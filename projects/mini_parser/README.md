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
- [x] **03. 構文解析器（Parser） (`mini-parser/03-parser`)**
- [x] **04. 構文木走査（Visitor） (`mini-parser/04-visitor`)**

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

---

## 03. 構文解析器（Parser）の記録

### 3.1 基本概念
- **再帰下降構文解析（Recursive Descent Parsing）**:
  - 文法規則（EBNF）をそのまま相互再帰する関数群として実装する、直感的かつ強力な構文解析アルゴリズム。
  - 演算子の優先順位を関数の呼び出し階層で自然に解決する。
    - **`parse_expr`**: 最も優先度の低い足し算・引き算（`+`, `-`）
    - **`parse_term`**: 次に優先度の高い掛け算・割り算（`*`, `/`）
    - **`parse_factor`**: 最優先の単一値（数値 `Number`、単項マイナス、括弧式 `( expr )`）
- **左結合（Left Associativity）の実現**:
  - `while let Some(token) = self.tokens.peek()` ループにより、左から順に木を包み直す（`1 + 2 + 3` → `Add(Add(1, 2), 3)`）。
- **括弧の再帰的処理**:
  - `parse_factor` が `(` を検出した瞬間に、最上位の `parse_expr()` を再帰呼び出しすることで、括弧内の式が完全に 1 つの AST に集約されてから外側へ戻る。
- **単項マイナス（Unary Minus）の局所化**:
  - `parse_factor` に `Token::Minus => 0 - factor` のルールを 1 箇所追加するだけで、式の先頭・演算子右辺・多重マイナス（`--5`）まで全て自動対応。

### 3.2 学び・検証記録

#### [テスト設計と責務の分離]
- **AST 構造の直接検証**:
  - パーサのテストにおいて、計算結果（`eval()`）に依存せず `ast == Expr::Add(...)` と AST の木構造そのものを `assert_eq!` で比較することで、計算ロジックと構文解析ロジックのテスト責務を完全に分離。
- **純粋な単体テスト（Pure Unit Test）と DX の両立**:
  - `parse_str("1 + 2 * 3")` のような文字列ヘルパー（可読性・量産性重視）に加え、`Vec<Token>` を直接渡す `test_operator_precedence_pure` を実装し、Lexer に一切依存しないパーサ単体の独立性を証明。

#### [テスト実行結果]
- **テスト実行結果**:
  ```text
  running 10 tests
  test ast::tests::test_eval_add_and_multiply ... ok
  test ast::tests::test_eval_nested_tree ... ok
  test ast::tests::test_eval_simple_number ... ok
  test lexer::tests::test_tokenize_invalid_char ... ok
  test lexer::tests::test_tokenize_valid_input ... ok
  test parser::tests::test_missing_closing_paren_error ... ok
  test parser::tests::test_operator_precedence ... ok
  test parser::tests::test_operator_precedence_pure ... ok
  test parser::tests::test_parentheses_precedence ... ok
  test parser::tests::test_unary_minus ... ok

  test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
  ```
- **実証できたこと**:
  - 四則演算の優先順位（掛け算優先）、括弧の優先順位、単項マイナス、閉じ括弧なし構文エラーの全パターンが正確に動作することを実証。

---

## 04. 構文木走査（Visitor パターン）の記録

### 4.1 基本概念
- **Visitor パターンによる責務分離**:
  - ASTノード（データ構造）と、Linter や Formatter（走査・検査・出力アルゴリズム）の責務を完全に切り離すデザインパターン。
  - 新しい静的解析ルール（Lint規則）を追加する際も、AST本体のコードを変更することなく、独立した構造体と `Visitor` トレイトの実装を追加するだけで拡張可能（開閉原則）。
- **`walk_expr` と探索の自動化**:
  - 木構造の左右の子ノードを再帰的に辿る定型処理を共通ヘルパー関数 `walk_expr` に集約。
  - 各 Visitor は `visit_expr` で「自身が興味のある特定のノード（割り算 `Expr::Divide` など）」のみをフック（オーバーライド）し、最後に `walk_expr` を呼ぶだけで木全体を自動探索。
- **Rust の型システムと参照外し**:
  - `?Sized`: 参照（ポインタ）経由で扱うことで、コンパイル時にスタックサイズが不明な型（トレイトオブジェクト等）も柔軟に受け入れ可能。
  - `**right`: `&Expr` からマッチした `&Box<Expr>` の「借用参照 `&`」と「ヒープポインタ `Box`」の 2 枚の皮を順次剥ぎ取り（Deref）、実体である `Expr` を直接判定。
- **トレイトのスコープ規則**:
  - トレイトで定義されたメソッド（`visit_expr` 等）を呼び出すには、呼び出し側のスコープに `use crate::visitor::Visitor;` とトレイト自身を明示的にインポートする必要がある。

### 4.2 学び・検証記録

#### [実装] Visitor トレイトと具体的アナライザー
- **`ZeroDivisionLinter`**:
  - 式を実行（評価）することなく、構文木を走査して `Expr::Divide(_, right)` の右辺が `0`（`Expr::Number(0)`）である箇所を静的に検出して警告を発出（静的解析 / Lint）。
- **`NodeCounter`**:
  - 構文木全体を巡回し、訪問した全ノード数を正確に集計。

#### [正常系・異常系実証] 単体テストによる検証
- **検証コード**:
  ```rust
  #[test]
  fn test_zero_division_linter_detects_bug() {
      let buggy_ast = Expr::Divide(Box::new(Expr::Number(10)), Box::new(Expr::Number(0)));
      let mut linter = ZeroDivisionLinter::new();
      linter.visit_expr(&buggy_ast);
      assert_eq!(linter.diagnostics.len(), 1);
      assert!(linter.diagnostics[0].contains("0除算"));
  }

  #[test]
  fn test_node_counter_with_paren() {
      // (1 + 2) * 3 => Multiply(Add(1, 2), 3)
      // 括弧は木構造の階層そのものとして表現されるため、独立ノードとしては消滅（計5個）
      let ast = Expr::Multiply(
          Box::new(Expr::Add(Box::new(Expr::Number(1)), Box::new(Expr::Number(2)))),
          Box::new(Expr::Number(3)),
      );
      let mut counter = NodeCounter::new();
      counter.visit_expr(&ast);
      assert_eq!(counter.count, 5);
  }
  ```
- **テスト実行結果**:
  ```text
  running 14 tests
  test ast::tests::test_eval_add_and_multiply ... ok
  test ast::tests::test_eval_nested_tree ... ok
  test ast::tests::test_eval_simple_number ... ok
  test lexer::tests::test_tokenize_invalid_char ... ok
  test lexer::tests::test_tokenize_valid_input ... ok
  test parser::tests::test_missing_closing_paren_error ... ok
  test parser::tests::test_operator_precedence ... ok
  test parser::tests::test_operator_precedence_pure ... ok
  test parser::tests::test_parentheses_precedence ... ok
  test parser::tests::test_unary_minus ... ok
  test visitor::tests::test_node_counter ... ok
  test visitor::tests::test_node_counter_with_paren ... ok
  test visitor::tests::test_zero_division_linter_detects_bug ... ok
  test visitor::tests::test_zero_division_linter_passes_valid_code ... ok

  test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
  ```
- **実証できたこと**:
  - 静的解析（Linter）が実行時エラー（パニック）を起こす前に潜在バグを検出できることを確認。
  - AST（抽象構文木）において括弧等の構文記号が階層構造として吸収され、無駄なノードが存在しないことをノード数カウントで実証。
