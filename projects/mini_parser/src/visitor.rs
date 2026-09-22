use crate::ast::Expr;

// 構文木を走査するための共通トレイト
pub trait Visitor {
    fn visit_expr(&mut self, expr: &Expr) {
        // 式ノードを訪れたときのフック（デフォルトは子ノードへ再帰）
        walk_expr(self, expr);
    }
}

// 構文木の子ノードを自動で再帰的に巡回するヘルパー関数
pub fn walk_expr<V: Visitor + ?Sized>(visitor: &mut V, expr: &Expr) {
    match expr {
        Expr::Number(_) => {} // 数値ノードには子がない
        Expr::Add(left, right)
        | Expr::Subtract(left, right)
        | Expr::Multiply(left, right)
        | Expr::Divide(left, right) => {
            // 左右の子ノードを再帰的に巡回
            visitor.visit_expr(left);
            visitor.visit_expr(right);
        }
    }
}

// ===================================================
// 具体的な Visitor ①: 0除算を検出するミニ Linter
// ===================================================
pub struct ZeroDivisionLinter {
    // 検出された警告（Lint 違反メッセージ）のリスト
    pub diagnostics: Vec<String>,
}

impl ZeroDivisionLinter {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }
}

impl Visitor for ZeroDivisionLinter {
    fn visit_expr(&mut self, expr: &Expr) {
        // 割り算の右側が0の場合は警告を出す
        if let Expr::Divide(_, right) = expr {
            // right が数値リテラル 0 であるかチェック
            if let Expr::Number(0) = **right {
                self.diagnostics
                    .push("警告(ZeroDivision):: 0除算が検出されました".to_string());
            }
        }
        // 子ノードの巡回を続ける
        walk_expr(self, expr);
    }
}

// ===================================================
// 具体的な Visitor ②: 構文木のノード数を数えるアナライザー
// ===================================================
pub struct NodeCounter {
    pub count: usize,
}

impl NodeCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Visitor for NodeCounter {
    fn visit_expr(&mut self, expr: &Expr) {
        self.count += 1; // ノードを 1 つ訪れるたびにカウント
        walk_expr(self, expr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_division_linter_detects_bug() {
        // 10 / 0
        let buggy_ast = Expr::Divide(Box::new(Expr::Number(10)), Box::new(Expr::Number(0)));

        let mut linter = ZeroDivisionLinter::new();
        linter.visit_expr(&buggy_ast);

        assert_eq!(linter.diagnostics.len(), 1);
        assert!(linter.diagnostics[0].contains("0除算"));
    }

    #[test]
    fn test_zero_division_linter_passes_valid_code() {
        // 10 / 2 (0除算なし)
        let valid_ast = Expr::Divide(Box::new(Expr::Number(10)), Box::new(Expr::Number(2)));

        let mut linter = ZeroDivisionLinter::new();
        linter.visit_expr(&valid_ast);

        assert!(linter.diagnostics.is_empty());
    }

    #[test]
    fn test_node_counter() {
        // 1 + 2 * 3
        // ノード内訳: Add(1), Number(1), Multiply(1), Number(2), Number(3) = 計 5 個
        let ast = Expr::Add(
            Box::new(Expr::Number(1)),
            Box::new(Expr::Multiply(
                Box::new(Expr::Number(2)),
                Box::new(Expr::Number(3)),
            )),
        );

        let mut counter = NodeCounter::new();
        counter.visit_expr(&ast);

        assert_eq!(counter.count, 5);
    }

    //括弧はノード数に含まれない
    #[test]
    fn test_node_counter_with_paren() {
        // (1 + 2) * 3
        // ノード内訳: Multiply(1), Add(1), Number(1), Number(2), Number(3) = 計 5 個
        let ast = Expr::Multiply(
            Box::new(Expr::Add(
                Box::new(Expr::Number(1)),
                Box::new(Expr::Number(2)),
            )),
            Box::new(Expr::Number(3)),
        );

        let mut counter = NodeCounter::new();
        counter.visit_expr(&ast);

        assert_eq!(counter.count, 5);
    }
}
