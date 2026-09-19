mod config {
    pub fn get_port() -> u16 {
        8080
    }

    #[derive(Debug)]
    pub struct ServerConfig {
        pub host: String,
        port: u16, //private なので、外部からは直接触れない
    }

    impl ServerConfig {
        pub fn new(host: String, port: u16) -> Self {
            Self { host, port }
        }

        pub fn port(&self) -> u16 {
            self.port
        }
    }
}

// 同一ディレクトリにある`math.rs`をモジュールツリーに登録する
mod math;

// `use`キーワードで長いパスを短縮してスコープに持ち込む
use math::advanced;

fn main() {
    // ============================
    // 1. インラインモジュールと可視性
    // ============================
    println!("\n---- 1. インラインモジュールと可視性 ----");
    let port = config::get_port();
    println!("Port: {}", port);

    // ============================
    // 2. 構造体と可視性
    // ============================
    println!("\n---- 2. 構造体と可視性 ----");
    let config = config::ServerConfig::new("localhost".to_string(), 8080);
    println!("ServerConfig: {:?}", config);
    println!("config.host（直接アクセス）: {}", config.host);
    println!("config.port（メソッド経由アクセス）: {}", config.port());

    // ============================
    // 3. ファイル分割とモジュールパス
    // ============================
    println!("\n---- 3. ファイル分割とモジュールパス ----");
    let sum = math::add(1, 2);
    println!("Sum: {}", sum);

    let product = advanced::multiply(3, 4);
    println!("Product: {}", product);

    let doubled = advanced::add_and_doubule(3, 7);
    println!("Add & Doubule: {}", doubled);

    // ============================
    // 4. クレート内部専用関数
    // ============================
    println!("\n---- 4. クレート内部専用関数 ----");
    let result = math::inner_calc(5);
    println!("Inner calc: {}", result);
}
