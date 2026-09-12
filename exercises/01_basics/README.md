# 01_basics: 基本文法と変数

Rust の基本文法、変数の不変性（デフォルト）と可変性（`mut`）、シャドーイング、基本データ型について学ぶ演習です。

---

## 実行コマンド

```bash
# ビルド＆実行
cargo run -p basics

# テスト実行
cargo test -p basics
```

---

## 学び・詰まったポイント（Qiita下書きメモ）

### 1. `cargo new` でパッケージ名が数字から始められない
- **やりたかったこと**: `exercises/01_basics` というディレクトリ名でパッケージを作成しようとした
- **発生したエラー**:
  ```text
  error: invalid character `0` in package name: `01_basics`, the name cannot start with a digit
  note: the directory name is used as the package name
  help: to override the package name, pass `--name <pkgname>`
  ```
- **原因**:
  - Rust の識別子（クレート名・パッケージ名）は、C言語などと同様に「数字から始めることができない」命名規則がある。
  - `cargo new` はデフォルトで作成先ディレクトリ名をそのままパッケージ名に採用しようとするため。
- **解決法**:
  - ディレクトリ名は整理のため `01_basics` としつつ、`--name basics` を指定してパッケージ名だけ英字始まりにする。
  ```bash
  cargo new --bin --vcs none --name basics exercises/01_basics
  ```

---

<!-- 今後発生したエラーや気づきをここに追記していきます -->
