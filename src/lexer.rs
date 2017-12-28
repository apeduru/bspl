use std::u32;
use error::LexerError;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Symbol {
    OR,
    AND,
    XOR,
    NOT,
    RSHIFT,
    LSHIFT,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    Keyword(String),
    Decimal(String), // 42
    Hexadecimal(String), // 0x2a
    Operator(Symbol),
}

pub type Tokens = Vec<(usize, Token)>;

pub fn lexer(line: &str) -> Result<Tokens, LexerError> {
    let mut tokens = Tokens::new();

    let mut iterator = line.chars().enumerate().peekable();
    while let Some((position, character)) = iterator.next() {
        if character.is_whitespace() {
            continue;
        }

        match character {
            ')' => tokens.push((position, Token::CloseBracket)),
            '(' => tokens.push((position, Token::OpenBracket)),
            '^' => tokens.push((position, Token::Operator(Symbol::XOR))),
            '&' => tokens.push((position, Token::Operator(Symbol::AND))),
            '|' => tokens.push((position, Token::Operator(Symbol::OR))),
            '~' => tokens.push((position, Token::Operator(Symbol::NOT))),
            '>' => {
                let shift_position = position;
                match iterator.peek() {
                    Some(&(_, '>')) => {
                        iterator.next();
                        tokens.push((shift_position, Token::Operator(Symbol::RSHIFT)));
                    }
                    _ => return Err(LexerError::UnknownOperator(position)),
                }
            }
            '<' => {
                let shift_position = position;
                match iterator.peek() {
                    Some(&(_, '<')) => {
                        iterator.next();
                        tokens.push((shift_position, Token::Operator(Symbol::LSHIFT)));
                    }
                    _ => return Err(LexerError::UnknownOperator(position)),
                }
            }

            _ if character.is_alphanumeric() => {
                let radix_position = position;
                let mut radix = String::new();
                radix.push(character);
                while let Some(&(_, rx)) = iterator.peek() {
                    if !rx.is_alphanumeric() {
                        break;
                    }
                    iterator.next();
                    radix.push(rx);
                }

                if radix.parse::<u32>().is_ok() {
                    tokens.push((radix_position, Token::Decimal(radix)));
                } else if radix.as_str().starts_with("0x") &&
                          u32::from_str_radix(radix.as_str().split_at(2).1, 16).is_ok() {
                    tokens.push((radix_position, Token::Hexadecimal(radix)));
                } else if radix.chars().all(|c| c.is_alphabetic()) {
                    tokens.push((radix_position, Token::Keyword(radix)));
                } else {
                    return Err(LexerError::RadixError(position));
                }
            }
            _ => return Err(LexerError::UnknownOperator(position)),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use std::u32;
    use error::LexerError;
    use lexer::{lexer, Token, Tokens, Symbol};

    #[test]
    fn blank(){
        let expression = "";
        assert_eq!(lexer(expression).unwrap(), vec![]);
    }

    #[test]
    fn symbols(){
        let expression = "^&|~";
        let tokens: Tokens = vec![(0, Token::Operator(Symbol::XOR)),
                                  (1, Token::Operator(Symbol::AND)),
                                  (2, Token::Operator(Symbol::OR)),
                                  (3, Token::Operator(Symbol::NOT))];
        assert_eq!(lexer(expression).unwrap(), tokens);
    }

    #[test]
    fn expression_valid(){
        let expression = "12 | (1 << 12)";
        let tokens: Tokens = vec![(0, Token::Decimal("12".to_string())),
                                  (3, Token::Operator(Symbol::OR)),
                                  (5, Token::OpenBracket),
                                  (6, Token::Decimal("1".to_string())),
                                  (8, Token::Operator(Symbol::LSHIFT)),
                                  (11, Token::Decimal("12".to_string())),
                                  (13, Token::CloseBracket)];
        assert_eq!(lexer(expression).unwrap(), tokens);
    }

    #[test]
    fn expression_invalid(){
        let expression = "12 | ( << 12)";
        let tokens: Tokens = vec![(0, Token::Decimal("12".to_string())),
                                  (3, Token::Operator(Symbol::OR)),
                                  (5, Token::OpenBracket),
                                  (7, Token::Operator(Symbol::LSHIFT)),
                                  (10, Token::Decimal("12".to_string())),
                                  (12, Token::CloseBracket)];
        assert_eq!(lexer(expression).unwrap(), tokens);
    }


    #[test]
    fn keywords_multiple(){
        let expression = "exit help license bspl";
        let tokens: Tokens = vec![(0, Token::Keyword("exit".to_string())),
                                  (5, Token::Keyword("help".to_string())),
                                  (10, Token::Keyword("license".to_string())),
                                  (18, Token::Keyword("bspl".to_string()))];

        assert_eq!(lexer(expression).unwrap(), tokens);
    }

    #[test]
    fn symbol_invalid(){
        let expression = "^&%|~";
        assert_eq!(lexer(expression), Err(LexerError::UnknownOperator(2)));
    }

    #[test]
    fn decimal_max(){
        let expression = "4294967295";
        assert_eq!(lexer(expression).unwrap(),
                   vec![(0, Token::Decimal(u32::MAX.to_string()))]);
    }

    #[test]
    fn decimal_overflow(){
        let expression = "4294967296";
        assert_eq!(lexer(expression), Err(LexerError::RadixError(0)));
    }

    #[test]
    fn hexadecimal_max(){
        let expression = "0xffffffff";
        assert_eq!(lexer(expression).unwrap(),
                   vec![(0, Token::Hexadecimal("0xffffffff".to_string()))]);
    }

    #[test]
    fn hexadecimal_overflow(){
        let expression = "0xabcdefgh";
        assert_eq!(lexer(expression), Err(LexerError::RadixError(0)));
    }

    #[test]
    fn rshift_incomplete(){
        let expression = ">> > << ";
        assert_eq!(lexer(expression), Err(LexerError::UnknownOperator(3)));
    }

    #[test]
    fn rshift_multiple(){
        let expression = ">>>> >> >> ";
        let tokens: Tokens = vec![(0, Token::Operator(Symbol::RSHIFT)),
                                  (2, Token::Operator(Symbol::RSHIFT)),
                                  (5, Token::Operator(Symbol::RSHIFT)),
                                  (8, Token::Operator(Symbol::RSHIFT))];
        assert_eq!(lexer(expression).unwrap(), tokens);
    }

    #[test]
    fn lshift_incomplete(){
        let expression = "< <<";
        assert_eq!(lexer(expression), Err(LexerError::UnknownOperator(0)));
    }

    #[test]
    fn lshift_multiple(){
        let expression = "<< <<<< <<";
        let tokens: Tokens = vec![(0, Token::Operator(Symbol::LSHIFT)),
                                  (3, Token::Operator(Symbol::LSHIFT)),
                                  (5, Token::Operator(Symbol::LSHIFT)),
                                  (8, Token::Operator(Symbol::LSHIFT))];
        assert_eq!(lexer(expression).unwrap(), tokens);
    }

    #[test]
    fn brackets_incomplete(){
        let expression = "()(() ()";
        let tokens: Tokens = vec![(0, Token::OpenBracket),
                                  (1, Token::CloseBracket),
                                  (2, Token::OpenBracket),
                                  (3, Token::OpenBracket),
                                  (4, Token::CloseBracket),
                                  (6, Token::OpenBracket),
                                  (7, Token::CloseBracket)];
        assert_eq!(lexer(expression).unwrap(), tokens);
    }

    #[test]
    fn brackets_nested(){
        let expression = "((()))";
        let tokens: Tokens = vec![(0, Token::OpenBracket),
                                  (1, Token::OpenBracket),
                                  (2, Token::OpenBracket),
                                  (3, Token::CloseBracket),
                                  (4, Token::CloseBracket),
                                  (5, Token::CloseBracket)];
        assert_eq!(lexer(expression).unwrap(), tokens);
    }

}
