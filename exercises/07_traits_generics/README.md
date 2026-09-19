# 07_traits_generics: トレイトとジェネリクス（Traits & Generics）

特定の型に縛られずに再利用可能なコードを記述する「ジェネリクス（Generics）」と、ゼロコスト抽象化を支える「単相化（Monomorphization）」、共通の振る舞いを定義する「トレイト（Trait）」、および型に制約を与える「トレイト境界（Trait Bounds）」について学ぶ演習です。

---

## 実行コマンド

```bash
# トレイトとジェネリクス演習バイナリの実行
cargo run -p traits_generics

# コンパイルチェック
cargo check -p traits_generics
```

---

## 1. 基本事項・概念の整理

### 1.1 ジェネリクス（Generics: `<T>`）
コードの重複を減らし、異なる複数の型に対して同じロジックやデータ構造を適用するための仕組みです。

- **構造体でのジェネリクス**:
  ```rust
  struct Point<T, U> {
      x: T,
      y: U,
  }
  ```
  - 同一型だけでなく、複数の型パラメータ（`T`, `U`）を定義することで `Point<i32, i32>` や `Point<f64, &str>` など柔軟な組み合わせが可能。
- **メソッドでのジェネリクス**:
  - `impl<T, U> Point<T, U> { ... }` のように型パラメータを宣言してメソッドを定義する。
  - ゲッターの戻り値には参照 `&T` を使うことで、所有権を奪わずに安全に値を貸し出せる。

### 1.2 ゼロコスト抽象化：単相化（Monomorphization）
他言語（Java 等）のジェネリクスでは実行時の型消去やラッパーオブジェクトにより速度低下が生じることがありますが、Rust のジェネリクスは **実行時オーバーヘッドがゼロ（ゼロコスト抽象化）** です。

- **仕組み**:
  - コンパイル時にコードを静的解析し、実際に使われている具体的な型ごとに専用のコードを自動複製・生成（単相化）する。
  - 例えば `Point<i32, i32>` と `Point<f64, &str>` が使われていれば、コンパイラがそれぞれに特化した機械語を生成する。
  - 手書きで個別に型を作った場合と 100% 同等の最高速の実行パフォーマンスが保証される。

### 1.3 トレイト（`trait`）の基本
他言語の「インターフェース（Interface）」に相当し、型が共通して持つべき振る舞い（メソッドのシグネチャ）を定義する機能です。

- **トレイトの定義**:
  ```rust
  trait Summary {
      fn summarize(&self) -> String;
  }
  ```
- **各型への実装**:
  - `impl Summary for NewsArticle { ... }`
  - `impl Summary for Tweet { ... }`
  - 構造体ごとに異なる具体的な処理内容を記述する。

### 1.4 トレイト境界（Trait Bounds）
ジェネリックな型パラメータに対して、「特定のトレイトを実装している型に限定する」という制約を課す機能です。

#### 構文のバリエーション
1. **`impl Trait` 構文（糖衣構文）**:
   ```rust
   fn notify(item: &impl Summary) { ... }
   ```
   最も簡潔で直感的。引数ごとに個別の型を受け入れられる。
2. **本来のジェネリクス境界構文（`<T: Trait>`）**:
   ```rust
   fn notify<T: Summary>(item: &T) { ... }
   ```
   複数の引数が「同一の型 `T` であり、かつ `Summary` を実装していること」を強制したい場合に必須。
3. **複数トレイト境界（`+` 構文）**:
   ```rust
   fn notify<T: Summary + std::fmt::Display>(item: &T) { ... }
   ```
   複数の能力を同時に要求する。
4. **`where` 節**:
   ```rust
   fn some_function<T, U>(t: &T, u: &U) -> i32
   where
       T: Summary + Clone,
       U: Clone + std::fmt::Debug,
   { ... }
   ```
   型パラメータと制約が多くなった際に関数シグネチャの見通しを良くする。

### 1.5 これまでに学んだ標準トレイトの正体
Rust の多くの基本機能は、特別な言語組み込み機能ではなく **トレイトという統一ルール** の上に構築されています。

| トレイト名 | 役割 | コード例 |
| :--- | :--- | :--- |
| **`Debug`** | デバッグ用の文字列整形出力能力 | `#[derive(Debug)]`, `{:?}` |
| **`Display`** | ユーザー向けの文字列整形出力能力 | `{}` |
| **`Copy`** | スタック上の安価な自動ビットコピー能力 | プリミティブ型の暗黙コピー |
| **`Clone`** | 明示的なヒープを含むディープコピー能力 | `.clone()` |
| **`Drop`** | スコープ終了直前の自動クリーンアップ処理 | デストラクタの実装 |

---

## 2. 学び・検証記録（コード実装と挙動実証）

### 2.1 [ジェネリクス実証] 複数型パラメータを持つ Point の実装
- **検証コード**:
  ```rust
  #[derive(Debug)]
  struct Point<T, U> {
      x: T,
      y: U,
  }

  impl<T, U> Point<T, U> {
      fn get_x(&self) -> &T { &self.x }
      fn get_y(&self) -> &U { &self.y }
  }

  let p1 = Point { x: 5, y: 10 };
  let p2 = Point { x: 10.5, y: "Hello" };
  ```
- **実行結果**:
  ```text
  Point { x: 5, y: 10 }
  5
  10
  Point { x: 10.5, y: "Hello" }
  10.5
  "Hello"
  ```
- **実証できたこと**:
  - `Point` という単一の構造体定義から、整数同士（`i32, i32`）および小数と文字列スライス（`f64, &str`）という全く異なる型のインスタンスが型安全に生成できた。
  - ゲッターメソッドが参照（`&T`, `&U`）を返すことで、所有権をムーブさせずに値を取り出せることを確認。

### 2.2 [トレイト実証] Summary による共通インターフェースの実装
- **検証コード**:
  ```rust
  trait Summary {
      fn summarize(&self) -> String;
  }

  struct NewsArticle { headline: String, author: String }
  struct Tweet { username: String, content: String }

  impl Summary for NewsArticle {
      fn summarize(&self) -> String {
          format!("{} by {}", self.headline, self.author)
      }
  }

  impl Summary for Tweet {
      fn summarize(&self) -> String {
          format!("{}: {}", self.username, self.content)
      }
  }
  ```
- **実証できたこと**:
  - 記事（`NewsArticle`）とツイート（`Tweet`）という異なる内部構造を持つ型に対して、共通の振る舞い（`summarize`）を契約として強制できた。

### 2.3 [トレイト境界実証] notify 関数によるポリモーフィズム
- **検証コード**:
  ```rust
  fn notify(item: &impl Summary) {
      println!("速報: {}", item.summarize());
  }

  notify(&news_article);
  notify(&tweet);
  ```
- **実行結果**:
  ```text
  速報: 最新ニュースです by 田中
  速報: username: content
  ```
- **実証できたこと**:
  - `notify` 関数が `NewsArticle` と `Tweet` の両方を引数として受け入れ、それぞれの型に応じた `summarize` を正しく呼び出せる（ポリモーフィズム）ことを実証。
