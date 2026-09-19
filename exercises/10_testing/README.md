# 10_testing: テスト手法（Testing）

Rust の組み込みテストフレームワークを活用し、単体テスト（Unit Tests）と統合テスト（Integration Tests）、アサーションマクロ、異常系テスト（`#[should_panic]`）、`Result` を返すテスト、および並行実行モデルについて学ぶ演習です。

---

## 実行コマンド

```bash
# 全テストの実行（単体テスト＋統合テスト）
cargo test -p testing

# シリアル（単一スレッド）で順番に実行
cargo test -p testing -- --test-threads=1

# 特定のテストのみを名前で絞り込んで実行
cargo test -p testing -- test_divide_by_zero
```

---

## 1. 基本事項・概念の整理

### 1.1 単体テスト（Unit Tests）と `#[cfg(test)]`
- **`#[cfg(test)]` によるゼロコスト化**:
  - `cargo test` の実行時のみコンパイル対象とし、製品版ビルド（`cargo build --release` 等）からはテストコードやアサーションを完全に除外する条件付きコンパイルのアトリビュート。
  - 製品バイナリのサイズや実行速度に一切悪影響を与えない。
- **非公開（Private）関数のテスト容易性**:
  - `mod tests { use super::*; ... }` と記述することで、親モジュールにある `pub` のついていない内部関数（`internal_multiply` など）も直接テスト可能。
  - 言語仕様として「子モジュールから親モジュールの要素はすべて見える」ため、他言語のようにリフレクションやテスト用公開メソッドを用意する必要がない。

### 1.2 アサーションマクロ
| マクロ | 役割 | コード例 |
| :--- | :--- | :--- |
| **`assert!(expr)`** | 式が `true` であることを検証 | `assert!(s.contains("Alice"))` |
| **`assert_eq!(left, right)`** | 2つの値が等しい（`==`）ことを検証 | `assert_eq!(result, 4)` |
| **`assert_ne!(left, right)`** | 2つの値が等しくない（`!=`）ことを検証 | `assert_ne!(result, 10)` |

※ いずれのマクロも、第2引数以降に `format!` と同じ構文でカスタムエラーメッセージを指定できる。

### 1.3 異常系テスト（`#[should_panic]`）
- 意図的に `panic!` を発生させる関数（境界値チェックや不正引数など）の挙動を検証する。
- **パニックが発生すれば「合格（ok）」**、正常終了してしまった場合は「失敗（FAILED）」となる。
- `#[should_panic(expected = "...")]` を付与することで、意図しない別の理由によるパニックを弾き、期待するエラーメッセージが含まれているかまで厳密に照合できる。

### 1.4 `Result<(), E>` を返すテスト
- テスト関数の戻り値として `Result<(), E>` を指定可能。
- テスト内で失敗する可能性のある処理に対して **`?` 演算子** を直接利用でき、`unwrap()` の連打を防いでクリーンにテストを記述できる。
- `Err` が返された時点で自動的にテスト失敗として扱われる。

### 1.5 テストランナーの並行実行モデル（マルチスレッド）
- `cargo test` はデフォルトで **全テストを別々のスレッドで並行（並列）実行** する。
- **重要な教訓**:
  - 実行順序は上から順ではなく非決定的なため、テスト間で状態や副作用（ファイル書き込み、環境変数など）を共有してはならない（各テストは完全独立である必要がある）。
  - 順次実行が必要な場合は `-- --test-threads=1` を指定する。

### 1.6 統合テスト（Integration Tests: `tests/`）
- プロジェクトルートの `tests/` ディレクトリ配下に配置する。
- Cargo は各統合テストファイルを **「完全に別の外部クレート」** としてコンパイルする。
- **特徴と目的**:
  - `use testing::{add, ...};` のように、ライブラリの外部利用者と全く同じ立場でインポートする。
  - 公開API（`pub`）のみしか呼び出せないため、「外部利用者の視点での完全なブラックボックステスト」が担保される。

---

## 2. 学び・検証記録（コード実装と挙動実証）

### 2.1 [失敗体験実証] `assert_eq!` 不一致時の詳細レポート
期待値を `5`、実際の値を `4` にして意図的にテストを失敗させた際の出力。

- **失敗時のコンパイラ出力**:
  ```text
  ---- tests::it_works stdout ----
  thread 'tests::it_works' panicked at exercises/10_testing/src/lib.rs:12:9:
  assertion `left == right` failed
    left: 4
   right: 5
  ```
- **実証できたこと**:
  - 不一致箇所の実測値（`left`）と期待値（`right`）が分かりやすく整形表示されることを確認。

---

### 2.2 [単体テスト実証] 非公開関数とカスタムメッセージ
- **検証コード**:
  ```rust
  #[test]
  fn test_internal_multiply() {
      let result = internal_multiply(3, 4);
      assert_eq!(result, 12);
      assert_ne!(result, 10);
  }

  #[test]
  fn test_greeting_message() {
      let result = greeting("Alice");
      assert!(
          result.contains("Alice"),
          "挨拶文に`Alice`が含まれていません。実際の結果: {}",
          result
      );
  }
  ```
- **実証できたこと**:
  - `pub` を付与していない内部関数 `internal_multiply` を安全に単体テストできた。

---

### 2.3 [異常系・Result実証] パニック検証と Result 型テスト
- **検証コード**:
  ```rust
  #[test]
  #[should_panic(expected = "ゼロで割ることはできません")]
  fn test_divide_by_zero_should_panic() {
      divie(10, 0);
  }

  #[test]
  fn test_parse_number_success() -> Result<(), std::num::ParseIntError> {
      let result = parse_number("123");
      assert!(result.is_ok());
      assert_eq!(result.unwrap(), 123);
      Ok(())
  }

  #[test]
  fn test_parse_number_failure() -> Result<(), std::num::ParseIntError> {
      let result = parse_number("abc");
      assert!(result.is_err());
      Ok(())
  }
  ```
- **実行結果**:
  ```text
  test tests::test_divide_by_zero_should_panic - should panic ... ok
  test tests::test_parse_number_success ... ok
  test tests::test_parse_number_failure ... ok
  ```
- **実証できたこと**:
  - 0除算時に期待通りのメッセージでパニックし、テストがパスすることを確認。
  - パース成功／失敗の両ケースを `Result` を用いて型安全に検証。

---

### 2.4 [統合テスト実証] `tests/integration_test.rs`
- **検証コード**:
  ```rust
  use testing::{add, greeting};

  #[test]
  fn test_public_api_integration() {
      let sum = add(100, 200);
      assert_eq!(sum, 300);

      let message = greeting("Rustacean");
      assert_eq!(message, "Hello, Rustacean");
  }
  ```
- **実行結果**:
  ```text
  Running unittests src/lib.rs (target/debug/deps/testing-...)
  running 6 tests
  test tests::test_internal_multiply ... ok
  test tests::test_parse_number_failure ... ok
  test tests::test_parse_number_success ... ok
  test tests::it_works ... ok
  test tests::test_greeting_message ... ok
  test tests::test_divide_by_zero_should_panic - should panic ... ok

  test result: ok. 6 passed; 0 failed; ...

  Running tests/integration_test.rs (target/debug/deps/integration_test-...)
  running 1 test
  test test_public_api_integration ... ok

  test result: ok. 1 passed; 0 failed; ...
  ```
- **実証できたこと**:
  - 単体テスト（`src/lib.rs`）と統合テスト（`tests/integration_test.rs`）が別々のテストバイナリとして独立して実行されることを確認。
