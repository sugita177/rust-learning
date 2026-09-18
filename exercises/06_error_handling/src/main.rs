use std::fs::File;
use std::io::Read;

fn read_content_from_file() -> Result<String, std::io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    // ==============================
    // 1. panic!（回復不能なエラー）
    // ==============================
    println!("\n---- 1. panic! ----");
    // panic!("パニックの実験");
    let v = vec![1, 2, 3];
    println!("{}", v[0]);
    // 以下は範囲外のアクセスなのでパニックとなる
    // println!("{}", v[99]);

    // ===============================
    // 2. Result<T, E> と match
    // ===============================
    println!("\n---- 2. Result<T, E> と match ----");
    let greeting_file_result = File::open("hello.txt");

    match greeting_file_result {
        Ok(file) => {
            println!("ファイルオープン成功: {:?}", file);
        }
        Err(error) => {
            println!("ファイルオープン失敗: {:?}", error);
        }
    };

    // let f = File::open("hello.txt").unwrap();
    // println!("f: {:?}", f);
    // let g = File::open("hello.txt").expect("hello.txt が見つかりません");
    // println!("g: {:?}", g);

    // ===============================
    // 3. ? 演算子
    // ===============================
    println!("\n---- 3. ? 演算子 ----");
    // ファイルが存在しない場合
    let content_result = read_content_from_file();
    println!("content_result: {:?}", content_result);

    // ファイルが存在する場合
    std::fs::write("hello.txt", "こんにちは").expect("ファイル作成失敗");
    let content_result = read_content_from_file();
    println!("content_result: {:?}", content_result);
    std::fs::remove_file("hello.txt").expect("ファイル削除失敗");
}
