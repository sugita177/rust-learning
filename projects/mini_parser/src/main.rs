mod ast;
mod lexer;
mod parser;
mod visitor;

use ast::Expr;
use lexer::tokenize;
use parser::Parser;
use visitor::{NodeCounter, Visitor, ZeroDivisionLinter};

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

    // ============================
    // 構文木走査（Visitor / Linter）の動作確認
    // ============================
    println!("\n---- mini_parser: AST 走査と Linter ----");
    // 0除算バグが含まれた数式
    let buggy_input = "100 + (42 / 0) * 2";
    println!("検算する式: {}", buggy_input);

    let buggy_tokens = tokenize(buggy_input).expect("字句解析失敗");
    println!("トークン列: {:?}", buggy_tokens);

    let mut buggy_parser = Parser::new(buggy_tokens);
    let buggy_ast = buggy_parser.parse().expect("構文解析失敗");
    println!("AST 構造: {:?}", buggy_ast);

    //1. ノード数をカウントするVisitorを実行
    let mut counter = NodeCounter::new();
    counter.visit_expr(&buggy_ast);
    println!("構文木の総ノード数: {}", counter.count); // 7個

    // 2. 0除算を検出するLinterを実行
    let mut linter = ZeroDivisionLinter::new();
    linter.visit_expr(&buggy_ast);
    if linter.diagnostics.is_empty() {
        println!("Lint チェック合格: エラーはありません。");
    } else {
        println!(
            "Lint チェック不合格 ({} 件の警告):",
            linter.diagnostics.len()
        );
        for diag in &linter.diagnostics {
            println!("  {}", diag);
        }
    }
}
