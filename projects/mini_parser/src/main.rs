mod ast;

use ast::Expr;

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
}
