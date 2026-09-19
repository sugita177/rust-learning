// 公開関数
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// mathモジュールの中のサブモジュール
pub mod advanced {
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn add_and_doubule(a: i32, b: i32) -> i32 {
        // `super::`で親モジュールの関数を呼び出す
        super::add(a, b) * 2
    }
}

// クレート内部専用の関数
pub(crate) fn inner_calc(x: i32) -> i32 {
    x + 1
}
