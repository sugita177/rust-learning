# 06_error_handling: エラーハンドリング（Error Handling）

例外機構（`try-catch`）を持たない Rust において、エラーを「回復不能なエラー（`panic!`）」と「回復可能なエラー（`Result<T, E>`）」に厳密に区別して扱う設計思想、`match` 式による分岐、簡易メソッド（`unwrap` / `expect`）、および強力な `?` 演算子によるエラー伝播について学ぶ演習です。

---

## 実行コマンド

```bash
# エラーハンドリング演習バイナリの実行
cargo run -p error_handling

# コンパイルチェック
cargo check -p error_handling
```

---

## 1. 基本事項・概念の整理

### 1.1 回復不能なエラーと回復可能なエラーの区別
多くの言語ではあらゆるエラーを「例外（Exception）」として扱いますが、Rust はエラーを明確に 2 種類に区別します。

| 種類 | 意味 | Rust での表現 | 対処方法 |
| :--- | :--- | :--- | :--- |
| **回復不能なエラー**<br>*(Unrecoverable)* | プログラムのバグであり、安全に実行を続けられない致命的な状態 | **`panic!` マクロ** | スタックを巻き戻して（unwind）安全に即座終了させる |
| **回復可能なエラー**<br>*(Recoverable)* | 想定内の失敗であり、適切に対処（再試行・代替処理）すべき状態 | **`Result<T, E>` Enum** | `match` や `?` 演算子で型安全にハンドリングする |

### 1.2 回復不能なエラー：`panic!` マクロと境界チェック
- **手動でのパニック**: `panic!("エラーメッセージ");` を呼び出すと、その場でプロセスがクラッシュメッセージを出力して終了します。
- **配列の境界外アクセス**:
  - C言語などでは配列外アクセス時に不定なメモリを読み込み、セキュリティ脆弱性や未定義動作の原因になります。
  - Rust では実行時に境界チェック（Bounds Check）を行い、範囲外アクセスを検知した瞬間に安全に `panic!` してプログラムを停止させます。
- **到達不能コード（unreachable statement）の検知**:
  - `panic!` の直後にあるコードは絶対に実行されないため、コンパイラが静的解析で `warning: unreachable statement` を警告してくれます。

### 1.3 回復可能なエラー：`Result<T, E>` Enum と `match`
標準ライブラリで定義されている `Result` 列挙型を使って、成功と失敗を型として厳密に表現します。

```rust
enum Result<T, E> {
    Ok(T),  // 成功（型 T の値を保持）
    Err(E), // 失敗（型 E のエラー情報を保持）
}
```
- `File::open("hello.txt")` などの失敗しうる関数は、`Result<File, std::io::Error>` を返します。
- `match` 式を用いることで、成功時の処理（`Ok`）と失敗時の処理（`Err`）をコンパイルレベルで漏れなく強制されます。

### 1.4 簡易エラー処理：`unwrap()` と `expect()`
テストやプロトタイプなど、「失敗したら即座にパニックして終了して構わない」という場面のためのショートカットです。

- **`unwrap()`**: `Ok` なら中身を取り出し、`Err` なら自動で `panic!` する。
- **`expect("メッセージ")`**: `unwrap` と同様だが、**パニック時のエラーメッセージを開発者が明記** できる。
  - *実務のベストプラクティス*: 単なる `unwrap()` はログを見ても何が起きたか分かりにくいため、意図や前提条件を明記できる `expect()` を使うことが推奨される。

### 1.5 エラーの伝播（移譲）と `?` 演算子
関数内でエラーをその場で処理（握りつぶし）するのではなく、**呼び出し元（親）に関数を抜けてエラーをそのまま返却する** 処理を「エラーの伝播」と呼びます。

#### `match` による冗長な伝播
```rust
let mut file = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => return Err(e), // エラーなら関数を即座に抜けて呼び出し元に Err を返す
};
```

#### `?` 演算子によるエレガントな伝播
上記の 4 行の `match` は、**`?` たった 1 文字** に置き換えることができます。

```rust
let mut file = File::open("hello.txt")?;
```
- **`?` の動作ルール**:
  - 式の結果が `Ok(val)` の場合: 中身の `val` を安全に取り出して（アンラップして）代入し、次の行へ進む。
  - 式の結果が `Err(err)` の場合: **その瞬間に現在の関数全体から `return Err(From::from(err))` して即座に脱出する**。
- **前提条件**: `?` 演算子を使う関数は、戻り値の型が `Result`（または `Option`）である必要があります。

### 1.6 `std::io::Read::read_to_string` の設計思想
`file.read_to_string(&mut content)` は、引数として渡された可変参照（バッファ）に内容を流し込みます。
関数が新しい `String` を毎回ヒープに確保して返すのではなく、呼び出し側が用意したメモリ領域を再利用させることで、**無駄なメモリ再割り当てを防ぐ Rust のゼロコスト抽象化の設計** になっています。

---

## 2. 学び・検証記録（エラー体験と挙動実証）

### 2.1 [panic 実証] 手動 panic と配列外アクセスの安全な停止
- **検証コード**:
  ```rust
  // panic!("パニックの実験");
  let v = vec![1, 2, 3];
  println!("{}", v[99]);
  ```
- **実行結果**:
  ```text
  thread 'main' panicked at exercises/06_error_handling/src/main.rs:4:21:
  index out of bounds: the len is 3 but the index is 99
  ```
- **実証できたこと**:
  - メモリ破壊やセグメンテーションフォールトを起こすことなく、「長さ 3 に対してインデックス 99 にアクセスした」という明確な理由を出力して安全に強制終了した。

### 2.2 [Result 実証] 存在しないファイルオープン時の `Err` 分岐
- **検証コード**:
  ```rust
  let greeting_file_result = File::open("hello.txt");
  match greeting_file_result {
      Ok(file) => println!("ファイルオープン成功: {:?}", file),
      Err(error) => println!("ファイルオープン失敗: {:?}", error),
  }
  ```
- **実行結果**:
  ```text
  ファイルオープン失敗: Os { code: 2, kind: NotFound, message: "No such file or directory" }
  ```
- **実証できたこと**:
  - ファイルが存在しなくてもプログラム全体はクラッシュせず、`Err` アームでエラー情報（`ErrorKind::NotFound`）を安全に受け取って処理を続行できることを確認。

### 2.3 [注意点] `main` 関数内での `return;` による後続処理の中断
- **事象**: `Err(error) => { println!(...); return; }` と記述したところ、以降の「3. ? 演算子」のコードが一切実行されなくなった。
- **原因と学び**: `main` 関数の中で `return;` を呼ぶとプロセス自体がその時点で正常終了してしまうため。後続の検証を動かす場合は `return` せずに素通りさせるか、検証処理を独立した関数に切り出す必要がある。

### 2.4 [? 演算子実証] 失敗パターンと成功パターンの両面検証
- **検証コード**:
  ```rust
  fn read_content_from_file() -> Result<String, std::io::Error> {
      let mut file = File::open("hello.txt")?;
      let mut content = String::new();
      file.read_to_string(&mut content)?;
      Ok(content)
  }

  // 1. ファイルが存在しない場合
  let content_result = read_content_from_file();
  println!("content_result: {:?}", content_result);

  // 2. ファイルを作成して存在させた場合
  std::fs::write("hello.txt", "こんにちは").expect("ファイル作成失敗");
  let content_result = read_content_from_file();
  println!("content_result: {:?}", content_result);
  std::fs::remove_file("hello.txt").expect("ファイル削除失敗");
  ```
- **実行結果**:
  ```text
  content_result: Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
  content_result: Ok("こんにちは")
  ```
- **実証できたこと**:
  - **失敗時**: `File::open("hello.txt")?` の時点で自動的に早期リターンされ、`content_result` に `Err(NotFound)` が返った。
  - **成功時**: `?` が中身を安全に取り出して次の `read_to_string` に進み、最終的に `Ok("こんにちは")` が返った。
  - 後片付けとして `remove_file` を行うことで、リポジトリを汚さずに両方のパスを完全に実証できた。
