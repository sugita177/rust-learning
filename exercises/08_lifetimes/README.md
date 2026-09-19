# 08_lifetimes: ライフタイム（Lifetimes）

Rust における参照の安全性とダングリングポインタ（不正なメモリ領域への参照）の防止を静的（コンパイル時）に保証する「ライフタイム（Lifetimes）」と、ジェネリックライフタイム注釈（`'a`）、および「静的ライフタイム（`'static`）」について学ぶ演習です。

---

## 実行コマンド

```bash
# ライフタイム演習バイナリの実行
cargo run -p lifetimes

# コンパイルチェック
cargo check -p lifetimes
```

---

## 1. 基本事項・概念の整理

### 1.1 ライフタイムとは何か
ライフタイムとは、**参照が有効であるスコープ（生存期間）** のことです。
- **目的**: 参照先の実体が先に破棄（Drop）されて無効なメモリを指してしまう「ダングリングポインタ（Dangling Pointer）」をコンパイル時に完全に防ぐこと。
- ほとんどの場合、ライフタイムは借用チェッカー（Borrow Checker）によって暗黙的・自動的に推論されますが、複数の参照が関わり、戻り値の生存期間が曖昧になる場合には **明示的なライフタイム注釈** が必要になります。

### 1.2 ジェネリックライフタイム注釈（Generic Lifetime Parameters: `'a`）の本質
- **書き方**:
  ```rust
  fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() { x } else { y }
  }
  ```
- **極めて重要な原則（寿命を延ばすものではない）**:
  - ライフタイム注釈は、渡された値の**寿命を延ばすものではありません**。
  - コンパイラに対して「入力される `x` と `y`、および戻り値の参照は、いずれも **少なくともライフタイム `'a` の間は有効である**（共通の生存期間 `'a` を満たす）」という**関係性（制約・契約）**を表明するものです。
- **戻り値の実際の有効期間**:
  - 引数として渡された複数の参照のうち、**「より生存期間が短い方のスコープ」** に制約されます。

### 1.3 静的解析による最悪ケースの安全性保証
Rust のコンパイラは、関数の内部ロジックを実行せずにシグネチャの制約のみを元に呼び出し側を検査します。
- 例えば `longer(t1, t2)` で実行時の文字列長が `t1 > t2` であっても、コンパイラは「実行時には `t2`（短い寿命）が返される可能性がある」と仮定します。
- そのため、戻り値を受け取った変数を `t2` のスコープ外で利用しようとすると、即座にコンパイルエラーとして拒絶されます。

### 1.4 静的ライフタイム（`'static`）
プログラム全体の実行期間中、常に有効であることを表す特殊なライフタイムです。
```rust
let s: &'static str = "I have a static lifetime.";
```
- 文字列リテラル（`"..."`）は、コンパイル時に実行可能バイナリのデータ領域（読み取り専用セクション）に直接焼き込まれるため、プログラム起動から終了までメモリ上に常に存在し続けます。

### 1.5 （補足）ライフタイム省略規則（Lifetime Elision Rules）
すべての関数で毎回注釈を書く必要がないよう、コンパイラには決まった推論ルール（省略規則）が組み込まれています。
1. 各引数の参照には、それぞれ固有のライフタイムが割り当てられる（例: `fn f(x: &str, y: &str)` → `fn f<'a, 'b>(x: &'a str, y: &'b str)`）。
2. 入力参照が1つだけの場合、そのライフタイムがすべての出力参照に割り当てられる（例: `fn f(x: &str) -> &str` → `fn f<'a>(x: &'a str) -> &'a str`）。
3. メソッドの場合（引数に `&self` または `&mut self` がある場合）、`self` のライフタイムが出力参照に割り当てられる。

※ 複数の引数があり、どれが出力に結びつくか推論できない場合（今回の `longer` など）は規則 2・3 に該当しないため、手動注釈が必須になります。

---

## 2. 学び・検証記録（コンパイルエラーと挙動実証）

### 2.1 [コンパイルエラー実証] `E0106`: missing lifetime specifier
ライフタイム注釈を付けずに複数参照を受け取り、参照を返す関数を定義した場合のコンパイルエラー。

- **検証コード**:
  ```rust
  fn longer(x: &str, y: &str) -> &str {
      if x.len() > y.len() { x } else { y }
  }
  ```
- **コンパイラ出力**:
  ```text
  error[E0106]: missing lifetime specifier
   --> exercises/08_lifetimes/src/main.rs:1:32
    |
  1 | fn longer(x: &str, y: &str) -> &str {
    |              ----     ----     ^ expected named lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
  help: consider introducing a named lifetime parameter
    |
  1 | fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    |          ++++     ++          ++          ++
  ```
- **エラーの原因とコンパイラの意図**:
  - 戻り値の `&str` が `x` と `y` のどちらから借用されたものか、関数シグネチャだけでは判別できません。
  - 呼び出し側で戻り値の有効期間を検証できるようにするため、明示的な注釈 `<'a>` で関連付けを要求しています。

---

### 2.2 [コンパイルエラー実証] `E0597`: `t2` does not live long enough
異なるスコープ（寿命）を持つ2つの値を比較し、スコープ外で戻り値を利用しようとした場合のエラー。

- **検証コード**:
  ```rust
  let t1 = String::from("long string is long");
  let result2;
  {
      let t2 = String::from("short");
      result2 = longer(t1.as_str(), t2.as_str());
      println!("ブロック内: {}", result2);
  }
  // t2 はここで drop される
  println!("ブロック外: {}", result2); // コンパイルエラー！
  ```
- **コンパイラ出力**:
  ```text
  error[E0597]: `t2` does not live long enough
    --> exercises/08_lifetimes/src/main.rs:24:39
     |
  23 |         let t2 = String::from("short");
     |             -- binding `t2` declared here
  24 |         result2 = longer(t1.as_str(), t2.as_str());
     |                                       ^^ borrowed value does not live long enough
  25 |         println!("ブロック内: {}", result2);
  26 |     }
     |     - `t2` dropped here while still borrowed
  ...
  30 |     println!("ブロック外: {}", result2);
     |                                ------- borrow later used here
  ```
- **実証できたこと**:
  - `result2` には `longer` の契約に従い `t1` と `t2` のうち短い方（`t2` のスコープ）が割り当てられる。
  - たとえ実行時の長さ判定では `t1`（生存期間が長い）が返される結果であっても、コンパイラは静的解析の最悪ケースとして「`t2` を参照している可能性がある」と判断し、ダングリングポインタの発生を未然に遮断した。

---

### 2.3 [正常系実証] スコープ内での利用と静的ライフタイム
安全性が保証された範囲での実行結果。

- **実行結果**:
  ```text
  ----- 1. ライフタイム ----- 
  The longer string is abcd

  ----- 2. 借用チェックとライフタイムの異なる参照 ----- 
  ブロック内: long string is long
  long string is long

  ----- 3. 静的ライフタイム ----- 
  I have a static lifetime. from 37 line
  ```
- **実証できたこと**:
  - スコープ内（`t2` が生きている間）であれば、`longer` の戻り値を安全に出力可能。
  - 文字列リテラルは `'static` ライフタイムを持ち、プログラムの任意の場所で常に安全に参照できる。
