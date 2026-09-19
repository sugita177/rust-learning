fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // ===============================
    // 1. ライフタイム
    // ===============================
    println!("\n----- 1. ライフタイム ----- ");
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longer(s1.as_str(), s2);
    println!("The longer string is {}", result);

    // ================================
    // 2. 借用チェックとライフタイムの異なる参照
    // ================================
    println!("\n----- 2. 借用チェックとライフタイムの異なる参照 ----- ");
    let t1 = String::from("long string is long");
    let result2;
    {
        let t2 = String::from("short");
        result2 = longer(t1.as_str(), t2.as_str());
        println!("ブロック内: {}", result2);
    }
    println!("{}", t1);
    // result2のライフタイムはt2のライフタイムと同じなので、t2がスコープを抜けた後にresult2を参照しようとするとコンパイルエラーになる
    // result2がt1を参照していても同様
    // println!("ブロック外: {}", result2);

    // ===============================
    // 3. 静的ライフタイム
    // ===============================
    println!("\n----- 3. 静的ライフタイム ----- ");
    let s: &'static str = "I have a static lifetime.";
    println!("{} from {} line", s, line!());
}
