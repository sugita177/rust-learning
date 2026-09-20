mod ast;
mod lexer;

use ast::Expr;
use lexer::tokenize;

fn main() {
    println!("\n---- mini_parser: AST木構造 ----");

    // 数式: (1 + 2) * 3 を木構造として組み立てる
    let expr = Expr::Multiply(
        Box::new(Expr::Add(
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2)),
        )),
        Box::new(Expr::Number(3)),
    );

    println!("AST 構造: {:?}", expr);

    let result = expr.eval();
    println!("Result: {}", result);

    // ============================
    // 字句解析（Lexer）の動作確認
    // ============================
    println!("\n---- mini_parser: 字句解析（Lexer） ----");
    let input = "10 + 20 * (30 - 4)";
    println!("入力文字列: \"{}\"", input);

    match tokenize(input) {
        Ok(tokens) => {
            println!("トークン列: {:?}", tokens);
        }
        Err(e) => {
            println!("字句解析エラー: {:?}", e);
        }
    }
}
