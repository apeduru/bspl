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
    Decimal(String), // Literal expression: 42
    Radix(String), // Non-Dec
    Operator(Symbol),
    Unknown(char),
}

pub type Tokens = Vec<(usize, Token)>;

fn identify_radix(radix: String) -> Token {
    if !radix.parse::<i32>().is_err() {
        return Token::Decimal(radix);
    } else if radix.chars().all(|c| c.is_alphabetic()) {
        return Token::Variable(radix);
    } else {
        return Token::Radix(radix);
    }
}

pub fn lexer(line: &str) -> Tokens {
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
                    _ => tokens.push((shift_position, Token::Unknown(character))),
                }
            }
            '<' => {
                let shift_position = position;
                match iterator.peek() {
                    Some(&(_, '<')) => {
                        iterator.next();
                        tokens.push((shift_position, Token::Operator(Symbol::LSHIFT)));
                    }
                    _ => tokens.push((shift_position, Token::Unknown(character))),
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
                tokens.push((radix_position, identify_radix(radix)));
            }
            _ => tokens.push((position, Token::Unknown(character))),
        }
    }

    tokens
}
