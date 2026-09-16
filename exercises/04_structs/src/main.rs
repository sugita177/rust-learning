#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn show_color(c: &Color) {
    println!("r: {}", c.0);
    println!("g: {}", c.1);
    println!("b: {}", c.2);
}

fn show_point(p: &Point) {
    println!("x: {}", p.0);
    println!("y: {}", p.1);
    println!("z: {}", p.2);
    println!("point1 full = {:?}", p);
}

fn main() {
    // ===============================
    // 1. 構造体の基本
    // ===============================
    println!("\n---- 1. 構造体の基本 ----");
    let user1 = build_user(String::from("user"), String::from("user@example.com"));
    println!("---- user1 ----");
    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("sign_in_count: {}", user1.sign_in_count);
    println!("active: {}", user1.active);
    println!("user1 full = {:?}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("---- user2 ----");
    println!("username: {}", user2.username);
    println!("email: {}", user2.email);
    println!("sign_in_count: {}", user2.sign_in_count);
    println!("active: {}", user2.active);
    println!("user2 full = {:?}", user2);

    println!("---- user1 -----");
    // user2がuser1のusernameの所有権を取得したので、user1のusernameは参照できない。E0382
    // println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("sign_in_count: {}", user1.sign_in_count);
    println!("active: {}", user1.active);
    // user1.usernameの所有権が移動しているので、user1全体を表示しようとするとエラーになる。E0382
    // println!("user1 full = {:?}", user1);

    // ========================================
    // 2. タプル構造体
    // ========================================
    println!("\n---- 2. タプル構造体 ----");
    let color1 = Color(255, 0, 0);
    println!("---- color1 ----");
    show_color(&color1);
    println!("r: {}", color1.0);
    println!("g: {}", color1.1);
    println!("b: {}", color1.2);
    println!("color1 full = {:?}", color1);

    let point1 = Point(1, 2, 3);
    println!("---- point1 ----");
    show_point(&point1);
    println!("x: {}", point1.0);
    println!("y: {}", point1.1);
    println!("z: {}", point1.2);
    println!("point1 full = {:?}", point1);
    // Color型をPoint型として扱おうとすると、フィールドの型（i32）と個数は同じでも、型が異なるためコンパイルエラーになる。E0308
    // show_point(&color1);
}
