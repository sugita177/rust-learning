mod ast;
mod lexer;
mod parser;

use ast::Expr;
use lexer::tokenize;
use parser::Parser;

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

    // ============================
    // 構文解析（Parser）の動作確認
    // ============================
    println!("\n---- mini_parser: 完全パイプライン実行 ----");

    // 掛け算が先、括弧が最優先の計算式
    let input = "-10 + 20 * (30 - 4)";
    println!("入力数式: {}", input);

    // 1. 字句解析 (Lexer)
    let tokens = tokenize(input).expect("字句解析失敗");
    println!("トークン列: {:?}", tokens);

    // 2. 構文解析 (Parser)
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("構文解析失敗");
    println!("AST 構造: {:?}", ast);

    // 3. AST の評価 (eval)
    let result = ast.eval();
    println!("実行結果: {}", result);
    // 期待値: -10 + 20 * (30 - 4) = -10 + 20 * 26 = -10 + 520 = 510
}
