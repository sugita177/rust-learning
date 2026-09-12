# Rust Learning & Practice

Rustの言語仕様やコア概念（所有権・借用・ライフタイム・型システム）をゼロから実践的に学ぶための練習・学習リポジトリです。
将来的なOSS（[Biome](https://github.com/biomejs/biome) 等）へのコントリビューションを見据え、構文解析や静的解析に通じる基礎力を身につけることを目的としています。

---

## リポジトリ構成

本リポジトリは **Cargo ワークスペース（Workspace）** を採用しており、テーマごとに分割されたクレートを `exercises/` 配下で管理しています。

```text
.
├── Cargo.toml          # ワークスペース親設定
├── README.md           # 本ファイル
├── docs/               # 開発環境等のドキュメント
│   └── rust_macos_setup_guide.md
└── exercises/          # 各学習テーマのコード
    ├── 01_basics/      # 基本文法・変数・制御構文
    ├── 02_ownership/   # 所有権・ムーブ・借用
    └── ...
```

---

## 実行方法

ルートディレクトリから、任意のパッケージ（クレート）名を指定して直接ビルド・実行できます。

```bash
# 例: 01_basics を実行する場合
cargo run -p 01_basics

# テストを実行する場合
cargo test -p 01_basics

# ワークスペース全体のコード検査
cargo check
```

---

## 開発環境

- **言語**: Rust（最新のStableツールチェーン）
- **パッケージマネージャ**: Cargo
- 環境構築手順の詳細は [docs/rust_macos_setup_guide.md](docs/rust_macos_setup_guide.md) を参照してください。
