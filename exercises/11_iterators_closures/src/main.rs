fn main() {
    // ============================
    // 1. クロージャと環境のキャプチャ
    // ============================
    println!("---- 1. クロージャと環境のキャプチャ ----");
    // 不変借用（&）でのキャプチャ
    let text = String::from("Rust");
    let print_text = || {
        // 外側のtextを不変借用する
        println!("不変借用: {}", text);
    };

    print_text();
    print_text(); // 不変借用なので複数回呼べる
    println!("外側でもまだtextは有効: {}", text);

    // 可変借用（&mut）でのキャプチャ
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("可変借用: count = {}", count);
    };
    increment();
    increment();
    println!("最終的なcount: {}", count);

    // moveによる所有権の移動によるキャプチャ
    let data = vec![1, 2, 3];
    let consume_data = move || {
        println!("moveキャプチャ: {:?}", data);
    };
    consume_data();

    // 所有権が移動しているため、以下はコンパイルエラーになる。
    // println!("move後: data = {:?}", data);

    // 引数ありのクロージャー
    let add = |x, y| x + y;
    let sum = add(1, 2);
    println!("Sum: {}", sum);

    // ============================
    // 2. イテレータと遅延評価
    // ============================
    println!("\n---- 2. イテレータと遅延評価 ----");

    let numbers = vec![1, 2, 3, 4, 5];

    // 遅延評価
    // map を繋げただけでは、中のクロージャは一切実行されない
    println!("-- mapの定義直前 --");
    let lazy_iter = numbers.iter().map(|x| {
        println!("map実行中: {}", x);
        x * 2
    });
    println!("--mapの定義直後--");

    // 消費して初めてイテレーターの処理が走る（forループで消費）
    println!("--forループ開始--");
    for n in lazy_iter {
        println!("forループ内での値: {}", n);
    }
    println!("--forループ終了--");

    // iter_mutによる直接書き換え
    let mut mut_numbers = vec![10, 20, 30];
    for val in mut_numbers.iter_mut() {
        *val += 5; // 参照外しで直接加算
    }
    println!("iter_mut後の配列: {:?}", mut_numbers);

    // ============================================
    // 3. メソッドチェーンと collect、実用的パイプライン
    // ============================================
    println!("\n---- 3. メソッドチェーンと実用的パイプライン ----");
    let source = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // filter -> map -> collect の王道パターン
    // 偶数だけを抽出し、それぞれを 10 倍して新しい Vec に集約する
    let even_multiplied: Vec<i32> = source
        .iter()
        .filter(|&&x| x % 2 == 0) // filter には参照の参照（&&i32）が渡る
        .map(|&x| x * 10)
        .collect();
    println!("{:?}", even_multiplied);

    // 文字列から数値トークンだけを抽出して合計する
    let token_input = "100 42 invalid_token 58 -20 skipped_text 10";
    println!("\n入力トークン列: \"{}\"", token_input);

    // split_whitespace で空白区切り
    // filter_map でパース成功したもの（Ok）だけを取り出す
    // sum で一発集計
    let total: i32 = token_input
        .split_whitespace()
        .filter_map(|token| token.parse::<i32>().ok())
        .sum();
    println!("数値トークンの合計: {}", total);
}
