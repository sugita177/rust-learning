# Exercises (演習コード一覧)

このディレクトリは、Rustの言語機能やテーマごとに独立したパッケージ（バイナリクレート）を配置する場所です。
ルートの `Cargo.toml`（Workspace）で管理されているため、ビルド成果物はルートの `target/` に集約されます。

---

## ディレクトリ構成一覧（予定・実績）

| ディレクトリ | パッケージ名 | テーマ・学習内容 |
| :--- | :--- | :--- |
| `01_basics/` | `basics` | 基本文法、変数と可変性（`mut`）、シャドーイング、基本型 |
| `02_ownership/` | `ownership` | 所有権の移動（Move）、スコープとDrop、ヒープとスタック |
| `03_borrowing/` | `borrowing` | 参照（`&`）、可変参照（`&mut`）、借用チェッカーのルール |
| `04_structs/` | `structs` | 構造体の定義、メソッド（`impl`）、関連関数 |
| `05_enums/` | `enums` | 列挙型（`enum`）、パターンマッチ（`match` / `if let`） |
| `06_error_handling/` | `error_handling` | `Option<T>`、`Result<T, E>`、`?` 演算子 |
| `07_traits_generics/` | `traits_generics` | トレイト定義・実装、ジェネリクス型、トレイト境界 |
| `08_lifetimes/` | `lifetimes` | 参照の有効期間、ライフタイム注釈（`'a`） |

---

## 新しい演習パッケージの作成手順

新しいテーマを始める際は、**ルートディレクトリから** 以下のコマンドで作成します。

```bash
# --vcs none を指定して個別の .git 作成を防ぎます
cargo new --bin --vcs none exercises/01_basics
```

※作成後、ルートの `Cargo.toml` の `members = ["exercises/*"]` により、自動的にワークスペースへ認識されます。

---

## 実行方法

ルートディレクトリからパッケージ名を指定して実行できます。

```bash
# 実行
cargo run -p basics

# テスト実行
cargo test -p basics
```
