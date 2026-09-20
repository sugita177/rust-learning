// 数式を構成する最小単位（トークン）
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Number(i64), // 数値（例: 42）
    Plus,        // '+'
    Minus,       // '-'
    Asterisk,    // '*'
    Slash,       // '/'
    LParen,      // '('
    RParen,      // ')'
}
// 字句解析時のエラー
#[derive(Debug, PartialEq, Eq)]
pub enum LexError {
    // 認識できない不正な文字（例: '@' や 'a' など）
    UnexpectedChar(char),
}

// 文字列を受け取り、トークンの配列（Vec<Token>）に変換する関数
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // 空白文字（スペースやタブなど）は読み飛ばす
            ' ' | '\t' | '\r' | '\n' => {
                chars.next(); // 1文字進める
            }
            // 1文字の記号類
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Asterisk);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            // 数字（'0'..='9'）の連続を読み取る
            '0'..='9' => {
                let mut num_str = String::new();
                // 次の文字も数字である限り、ループして文字列に蓄積する
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        num_str.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                // 文字列を i64 にパース（数字しか集めていないので unwrap で安全）
                let num: i64 = num_str.parse().unwrap();
                tokens.push(Token::Number(num));
            }
            // 上記以外（アルファベットや記号など）はエラー
            _ => return Err(LexError::UnexpectedChar(ch)),
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_valid_input() {
        let input = " 1 + ( 23 * 456 ) / 7 - 89 ";
        let tokens = tokenize(input).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Plus,
                Token::LParen,
                Token::Number(23),
                Token::Asterisk,
                Token::Number(456),
                Token::RParen,
                Token::Slash,
                Token::Number(7),
                Token::Minus,
                Token::Number(89),
            ]
        )
    }

    #[test]
    fn test_tokenize_invalid_char() {
        let input = "10 + @ - 5";
        let result = tokenize(input);

        assert_eq!(result, Err(LexError::UnexpectedChar('@')));
    }
}
