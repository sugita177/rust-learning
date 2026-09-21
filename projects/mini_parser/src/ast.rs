// 数式（Expression）を表す構文木ノード
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    // 末端のノード（数値リテラル: 42 など）
    Number(i64),

    // Box<Expr> でヒープポインタ化してサイズを固定
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> i64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Add(left, right) => left.eval() + right.eval(),
            Expr::Subtract(left, right) => left.eval() - right.eval(),
            Expr::Multiply(left, right) => left.eval() * right.eval(),
            Expr::Divide(left, right) => left.eval() / right.eval(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_simple_number() {
        let expr = Expr::Number(42);
        assert_eq!(expr.eval(), 42);
    }

    #[test]
    fn test_eval_add_and_multiply() {
        // (1 + 2) * 3 = 9
        let expr = Expr::Multiply(
            Box::new(Expr::Add(
                Box::new(Expr::Number(1)),
                Box::new(Expr::Number(2)),
            )),
            Box::new(Expr::Number(3)),
        );
        assert_eq!(expr.eval(), 9);
    }

    #[test]
    fn test_eval_nested_tree() {
        // 1 + (2 * 3) = 7
        let expr = Expr::Add(
            Box::new(Expr::Number(1)),
            Box::new(Expr::Multiply(
                Box::new(Expr::Number(2)),
                Box::new(Expr::Number(3)),
            )),
        );
        assert_eq!(expr.eval(), 7);
    }
}
