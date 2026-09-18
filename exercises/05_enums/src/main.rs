#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        println!("{:?}", self);
    }

    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit: 終了します");
            }
            Message::Move { x, y } => {
                println!("Move: ({}, {})へ移動します", x, y);
            }
            Message::Write(text) => {
                println!("Write: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("ChangeColor: ({}, {}, {})へ変更します", r, g, b);
            }
        }
    }

    fn call_partial(&self) {
        match self {
            Message::Quit => println!("終了"),
            _ => println!("終了以外のメッセージ"),
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // ===============================
    // 1. Enumの基本
    // ===============================
    println!("\n---- 1. Enumの基本 ----");
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 2 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);

    m1.print();
    m2.print();
    m3.print();
    m4.print();

    // ===============================
    // 2. match式
    // ===============================
    println!("\n---- 2. match式 ----");
    m1.call();
    m2.call();
    m3.call();
    m4.call();
    m1.call_partial();
    m2.call_partial();
    m3.call_partial();
    m4.call_partial();

    // ===============================
    // 3. Option<T>
    // ===============================
    println!("\n---- 3. Option<T> ----");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six = {:?}", six);
    println!("none = {:?}", none);
    if let Some(x) = six {
        println!("{}", x);
    }
    if let None = none {
        println!("None");
    }
}
