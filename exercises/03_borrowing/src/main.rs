// 不変参照による借用
fn get_length(s_ref: &String) -> usize {
    println!("get_length内での引数: {}", s_ref);
    println!("get_length内での引数（s_refが指す先）のアドレス: {:p}", s_ref);
    println!("get_length内での引数（s_ref自身）のスタックアドレス: {:p}", &s_ref);
    println!(
        "ヒープ領域にあるget_length内での引数が指す領域のアドレス: {:p}",
        s_ref.as_ptr()
    );
    s_ref.len()
}

// 可変参照による借用
fn change_str(s_ref: &mut String) {
    s_ref.push_str(", world");
}

// ローカル変数の参照を返そうとする関数
// （ローカル変数 s はスコープ終了（}）時に Drop されてメモリから消滅するため、
//  その参照を関数の外に返そうとするとダングリングポインタ（未定義動作）になる。
//  Rust コンパイラはこれを検知し、missing lifetime specifier（E0106）としてコンパイルを拒否する）
//fn dangle() -> &String {
//    let s = String::from("dangle");
//    &s // ローカル変数 s の参照を返す
//}

// 所有権を直接返す安全な関数（所有権が呼び出し元へムーブする）
fn no_dangle() -> String {
    let s = String::from("no_dangle");
    s // 参照ではなく、所有権そのものを返す
}

fn main() {
    // ==========================================
    // 1. 不変参照による借用
    // ==========================================
    println!("\n--- 1. 不変参照による借用 ---");
    let s1 = String::from("Hello");
    println!("s1: {}", s1);
    println!("s1自体のメモリアドレス: {:p}", &s1);
    println!("ヒープ領域にあるs1が指す領域のアドレス: {:p}", s1.as_ptr());
    let len = get_length(&s1);
    println!("len: {}", len);
    println!("s1 after get_length: {}", s1);
    println!("s1自体のメモリアドレス: {:p}", &s1);
    println!("ヒープ領域にあるs1が指す領域のアドレス: {:p}", s1.as_ptr());

    // ==========================================
    // 2. 可変参照による借用
    // ==========================================
    println!("\n--- 2. 可変参照による借用 ---");
    let mut s2 = String::from("Hello");
    println!("s2: {}", s2);
    println!("s2自体のメモリアドレス: {:p}", &s2);
    println!("ヒープ領域にあるs2が指す領域のアドレス: {:p}", s2.as_ptr());
    change_str(&mut s2);
    println!("s2 after change_str: {}", s2);
    println!("s2自体のメモリアドレス: {:p}", &s2);
    println!("ヒープ領域にあるs2が指す領域のアドレス: {:p}", s2.as_ptr());

    // ==========================================
    // 3. 借用チェッカー（ルール①（排他性・NLL））
    // ==========================================
    println!("\n--- 3. 借用チェッカー（ルール①（排他性・NLL））---");
    let mut s3 = String::from("Rust");

    let r1 = &mut s3;
    // 一度に一つの可変参照のみ可能なので、下記のコメントアウトを外すとコンパイルエラーになる（E0499）
    // let r2 = &mut s3;
    println!("r1: {}", r1);
    // println!("r2: {}", r2);

    // 可変参照 r1 はこの println! を最後に二度と使われないため、
    // 借用チェッカー（NLL）によりここで借用期間が終了したとみなされる。
    // そのため、次の行で新しい可変参照 r3 を作ることが許可される。
    let r3 = &mut s3;
    println!("r3: {}", r3);

    let mut s4 = String::from("Cargo");
    let r4 = &s4;
    // 不変参照は複数同時に存在できる
    let r5 = &s4;
    // 可変参照は一度に一つのものしか存在できないので、下記のコメントアウトを外すとコンパイルエラーになる（E0502）
    // let r6 = &mut s4;
    println!("r4: {}", r4);
    println!("r5: {}", r5);
    // println!("r6: {}", r6);

    // 不変参照 r4, r5 の最後の使用（println!）が終わり借用期間が終了したため、可変参照 r6 が有効になる
    let r6 = &mut s4;
    println!("r6: {}", r6);

    // ==========================================
    // 4. 借用チェッカー（ルール②（ダングリング参照の防止）
    // ==========================================
    println!("\n--- 4. 借用チェッカー（ルール②（ダングリング参照の防止）---");
    // let reference_to_nothing = dangle();
    // println!("{}", reference_to_nothing);

    let some_string = no_dangle();
    println!("some_string: {}", some_string);
}
