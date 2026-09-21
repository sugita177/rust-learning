use crate::ast::Expr;
use crate::lexer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

// 構文解析時のエラー
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedEof,
    UnexpectedToken(Token),
    MissingClosingParen,
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    // パース開始エントリーポイント
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expr()?;

        // パース終了後に余分なトークンがないかチェック
        if let Some(token) = self.tokens.next() {
            return Err(ParseError::UnexpectedToken(token));
        }

        Ok(expr)
    }

    // ① 式 (Expression): 足し算・引き算（+ , -）を処理
    // 文法: expr = term (('+' | '-') term)*
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;

        // + or - が続く限りループ
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Plus => {
                    self.tokens.next();
                    let right = self.parse_term()?;
                    left = Expr::Add(Box::new(left), Box::new(right));
                }
                Token::Minus => {
                    self.tokens.next();
                    let right = self.parse_term()?;
                    left = Expr::Subtract(Box::new(left), Box::new(right));
                }
                _ => break, // どちらでもないならループ終了
            }
        }
        Ok(left)
    }

    // ② 項 (Term): 掛け算・割り算（* , /）を処理
    // 文法: term = factor (('*' | '/') factor)*
    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_factor()?;

        // * or / が続く限りループ
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Asterisk => {
                    self.tokens.next();
                    let right = self.parse_factor()?;
                    left = Expr::Multiply(Box::new(left), Box::new(right));
                }
                Token::Slash => {
                    self.tokens.next();
                    let right = self.parse_factor()?;
                    left = Expr::Divide(Box::new(left), Box::new(right));
                }
                _ => break, // どちらでもないならループ終了
            }
        }
        Ok(left)
    }

    // ③ 因子 (Factor): 最優先の「数値」または「( 括弧式 )」を処理
    // 文法: factor = NUMBER | '(' expr ')'
    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        match self.tokens.next() {
            Some(Token::Number(n)) => Ok(Expr::Number(n)),

            // 単項マイナスの処理（-x を 0 - x として扱う）
            Some(Token::Minus) => {
                let expr = self.parse_factor()?;
                Ok(Expr::Subtract(Box::new(Expr::Number(0)), Box::new(expr)))
            }

            Some(Token::LParen) => {
                // '('を消費したので、再帰的に式をパース
                let expr = self.parse_expr()?;

                // 文法的に正しければ')'が来るはずなので、それを消費
                match self.tokens.next() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err(ParseError::MissingClosingParen),
                }
            }
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::UnexpectedEof),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    // ヘルパー関数: 文字列から一発で AST を構築する
    fn parse_str(input: &str) -> Result<Expr, ParseError> {
        let tokens = tokenize(input).unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse()
    }
    #[test]
    fn test_operator_precedence() {
        // 1 + 2 * 3 => Add(1, Multiply(2, 3))
        let ast = parse_str("1 + 2 * 3").unwrap();
        assert_eq!(
            ast,
            Expr::Add(
                Box::new(Expr::Number(1)),
                Box::new(Expr::Multiply(
                    Box::new(Expr::Number(2)),
                    Box::new(Expr::Number(3)),
                ))
            )
        );
    }
    #[test]
    fn test_parentheses_precedence() {
        // (1 + 2) * 3 => Multiply(Add(1, 2), 3)
        let ast = parse_str("(1 + 2) * 3").unwrap();
        assert_eq!(
            ast,
            Expr::Multiply(
                Box::new(Expr::Add(
                    Box::new(Expr::Number(1)),
                    Box::new(Expr::Number(2)),
                )),
                Box::new(Expr::Number(3)),
            )
        );
    }
    #[test]
    fn test_unary_minus() {
        // -5 * 3 => Multiply(Subtract(0, 5), 3)
        let ast = parse_str("-5 * 3").unwrap();
        assert_eq!(
            ast,
            Expr::Multiply(
                Box::new(Expr::Subtract(
                    Box::new(Expr::Number(0)),
                    Box::new(Expr::Number(5)),
                )),
                Box::new(Expr::Number(3)),
            )
        );
    }
    #[test]
    fn test_missing_closing_paren_error() {
        let result = parse_str("(1 + 2");
        assert_eq!(result, Err(ParseError::MissingClosingParen));
    }

    #[test]
    fn test_operator_precedence_pure() {
        // [1, +, 2, *, 3] を直接渡す（tokenize に一切依存しない！）
        let tokens = vec![
            Token::Number(1),
            Token::Plus,
            Token::Number(2),
            Token::Asterisk,
            Token::Number(3),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Expr::Add(
                Box::new(Expr::Number(1)),
                Box::new(Expr::Multiply(
                    Box::new(Expr::Number(2)),
                    Box::new(Expr::Number(3)),
                ))
            )
        );
    }
}
