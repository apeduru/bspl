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
    Variable(String),
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

                if !radix.parse::<u32>().is_err() {
                    tokens.push((radix_position, Token::Decimal(radix)));
                } else if radix.as_str().starts_with("0x") &&
                          u32::from_str_radix(radix.as_str().split_at(2).1, 16).is_ok() {
                    tokens.push((radix_position, Token::Hexadecimal(radix)));
                } else if radix.chars().all(|c| c.is_alphabetic()) {
                    tokens.push((radix_position, Token::Variable(radix)));
                } else {
                    return Err(LexerError::RadixError(position));
                }
            }
            _ => return Err(LexerError::UnknownOperator(position)),
        }
    }

    Ok(tokens)
}
