fn my_func_int(some_integer: i32) {
    println!("my_func_int内での引数: {}", some_integer);
}

fn my_func_str(some_string: String) {
    println!("my_func_str内での引数: {}", some_string);
    println!("my_func_str内での引数のメモリアドレス: {:p}", &some_string);
    println!(
        "ヒープ領域にあるsome_stringが指す領域のアドレス: {:p}",
        some_string.as_ptr()
    );
}

fn my_func_return_str(some_string: String) -> String {
    println!("my_func_return_str内での引数: {}", some_string);
    println!(
        "my_func_return_str内での引数のメモリアドレス: {:p}",
        &some_string
    );
    println!(
        "ヒープ領域にあるsome_stringが指す領域のアドレス: {:p}",
        some_string.as_ptr()
    );
    some_string
}

struct CustomDrop {
    name: &'static str,
}

impl Drop for CustomDrop {
    fn drop(&mut self) {
        println!("[Drop] {} が解放されました", self.name);
    }
}

fn main() {
    // ==========================================
    // 1. 所有権の移動（ムーブセマンティクス）
    // ==========================================
    println!("\n--- 1. 所有権の移動（ムーブセマンティクス） ---");
    let s1 = String::from("hello");
    println!("s1 = {}", s1);
    println!("s1自体のメモリアドレス: {:p}", &s1);
    println!("ヒープ領域にあるs1が指す領域のアドレス: {:p}", s1.as_ptr());
    //ここでs1が束縛する"hello"の所有権がs2に移る（ムーブセマンティクス）
    let s2 = s1;
    println!("s2 = {}", s2);
    println!("s2自体のメモリアドレス: {:p}", &s2);
    println!("ヒープ領域にあるs2が指す領域のアドレス: {:p}", s2.as_ptr());
    //この時点でs1には所有権がないため、println!マクロがs1を参照できず、コンパイルエラー
    // println!("s1 = {}", s1);

    // ==========================================
    // 2. クローン（Clone）
    // ==========================================
    println!("\n--- 2. クローン（Clone） ---");
    let s3 = String::from("Good morning");
    println!("s3 = {}", s3);
    println!("s3自体のメモリアドレス: {:p}", &s3);
    println!("ヒープ領域にあるs3が指す領域のアドレス: {:p}", s3.as_ptr());
    let s4 = s3.clone();
    println!("s4 = {}", s4);
    println!("s4自体のメモリアドレス: {:p}", &s4);
    println!("ヒープ領域にあるs4が指す領域のアドレス: {:p}", s4.as_ptr());
    //この時点でs3には所有権があるため、println!マクロがs3を参照できる
    println!("s3 = {}", s3);

    // ==========================================
    // 3. コピー（Copy : コピーセマンティクス）
    // ==========================================
    println!("\n--- 3. コピー（Copy : コピーセマンティクス） ---");
    // i32 は Copy トレイトを実装しているため、所有権のムーブではなくコピーセマンティクスが適用される
    let x = 5;
    let y = x;
    println!("x = {}, xのアドレス: {:p}", x, &x);
    println!("y = {}, yのアドレス: {:p}", y, &y);
    //この時点でxには所有権があるため、println!マクロがxを参照できる
    println!("x = {}", x);

    // ==========================================
    // 4. 関数と所有権
    // ==========================================
    println!("\n--- 4. 関数と所有権 ---");
    let s1 = String::from("hello");
    my_func_str(s1);
    // この時点でs1には所有権がないため、println!マクロがsを参照できず、コンパイルエラー
    // println!("{}", s1);

    let s2 = String::from("Good morning");
    println!("main関数でのs2: {}", s2);
    println!("s2自体のメモリアドレス: {:p}", &s2);
    println!("ヒープ領域にあるs2が指す領域のアドレス: {:p}", s2.as_ptr());
    let s2_returned = my_func_return_str(s2);
    // この時点でs2は所有権をムーブしているため、println!マクロがs2を参照できず、コンパイルエラー
    // println!("main関数でのs2: {}", s2);
    println!("main関数でのs2_returned: {}", s2_returned);
    println!("s2_returned自体のメモリアドレス: {:p}", &s2_returned);
    println!(
        "ヒープ領域にあるs2_returnedが指す領域のアドレス: {:p}",
        s2_returned.as_ptr()
    );

    let x = 5;
    my_func_int(x);
    // この時点でxには所有権があるため、println!マクロがxを参照できる
    println!("main関数でのx: {}", x);

    // ==========================================
    // 5. 所有権のドロップ（Drop）
    // ==========================================
    println!("\n--- 5. 所有権のドロップ（Drop）とスコープ ---");
    // 内部スコープでの解放
    println!("--- 内部スコープでの解放");
    {
        println!("内部スコープに入りました");
        let c_inner = CustomDrop { name: "c_inner" };
        println!("c_inner.name = {}", c_inner.name);
        println!("内部スコープから抜ける直前")
    }
    println!("内部スコープから抜けた直後");

    // 所有権移動時の解放
    println!("--- 所有権移動時の解放");
    let c_before_move = CustomDrop {
        name: "c_before_move",
    };
    println!("c_before_move.name = {}", c_before_move.name);
    println!("c_before_moveの所有権をc_after_moveへ移動する直前");
    let c_after_move = c_before_move;
    println!("c_before_moveからc_after_moveへの所有権移動の完了");
    println!("c_after_move.name = {}", c_after_move.name);

    // 早期解放
    println!("--- 早期解放");
    let c_early_drop = CustomDrop {
        name: "c_early_drop",
    };
    println!("c_early_drop.name = {}", c_early_drop.name);
    println!("c_early_dropを早期解放");
    drop(c_early_drop);
    println!("c_early_dropの解放完了");
}
