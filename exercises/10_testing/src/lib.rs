pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// 非公開関数
#[allow(dead_code)]
fn internal_multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

// 0除算時にpanic!が発生する関数
pub fn divie(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("ゼロで割ることはできません")
    }
    a / b
}

// Resultを返す関数
pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_internal_multiply() {
        let result = internal_multiply(3, 4);
        assert_eq!(result, 12);
        assert_ne!(result, 10);
    }

    #[test]
    fn test_greeting_message() {
        let result = greeting("Alice");
        assert!(
            result.contains("Alice"),
            "挨拶文に`Alice`が含まれていません。実際の結果: {}",
            result
        );
    }

    // panic!が発生することを確認するテスト
    #[test]
    #[should_panic(expected = "ゼロで割ることはできません")]
    fn test_divide_by_zero_should_panic() {
        divie(10, 0);
    }

    // Resultを返すテスト
    #[test]
    fn test_parse_number_success() -> Result<(), std::num::ParseIntError> {
        let result = parse_number("123");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 123);
        Ok(())
    }

    #[test]
    fn test_parse_number_failure() -> Result<(), std::num::ParseIntError> {
        let result = parse_number("abc");
        assert!(result.is_err());
        Ok(())
    }
}
