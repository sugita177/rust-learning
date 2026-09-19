use testing::{add, greeting};

#[test]
fn test_public_api_integration() {
    let sum = add(100, 200);
    assert_eq!(sum, 300);

    let message = greeting("Rustacean");
    assert_eq!(message, "Hello, Rustacean");
}
