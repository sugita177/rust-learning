# Projects (実践プロジェクト一覧)

このディレクトリは、Phase 1 で学んだ言語仕様（所有権、Enum、トレイト、モジュール、テスト、イテレータ等）を総合的に活用し、実用的なツールやOSS（Biome）のコア技術を模した本格プログラムを開発する場所です。

---

## 開発プロジェクト一覧

### 1. `mini_parser`: 数式パーサ＆AST走査エンジン

プログラミング言語のコンパイラやコード解析ツール（Biome）で使われる **「字句解析（Lexer）→ 構文解析（Parser）→ 抽象構文木（AST）→ 走査（Visitor）」** の一連のパイプラインを自作するプロジェクトです。

#### ディレクトリ・モジュール構成（完成イメージ）

```text
projects/mini_parser/
 ├── Cargo.toml
 ├── README.md
 └── src/
      ├── main.rs       # CLIエントリポイント
      ├── ast.rs        # 構文木ノード定義（enum Expr と Box<Expr>）
      ├── lexer.rs      # 字句解析器（文字列を Token 列へ変換）
      ├── parser.rs     # 構文解析器（Token 列から AST を構築）
      └── visitor.rs    # 構文木走査器（計算評価・簡易Linter等の処理）
```

#### 開発ステップ

| ステップ | ブランチ名 | 実装内容 |
| :--- | :--- | :--- |
| **01. 木構造とAST定義** | `mini-parser/01-tree-structure` | `enum Expr` と `Box<T>` による木構造表現、再帰的評価（`eval`）の実装 |
| **02. 字句解析器（Lexer）** | `mini-parser/02-lexer` | 文字列をトークン（数値、演算子、括弧）に分解するイテレータの実装 |
| **03. 構文解析器（Parser）** | `mini-parser/03-parser` | 再帰下降構文解析によるトークン列から AST への自動変換 |
| **04. 構文木走査（Visitor）** | `mini-parser/04-visitor` | Trait による Visitor パターンの実装、構文木の走査と静的チェック |

---

## 実行コマンド

```bash
# 実行
cargo run -p mini_parser

# テスト実行
cargo test -p mini_parser
```
