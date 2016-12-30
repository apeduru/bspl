use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    // Variable, must strictly be alphabetical
    Variable(String),
    // Expression, must appear after '='
    Expression(String),
    // Literal expression: 42
    Decimal(String),
    // Literal expression: 0b101010
    Binary(String),
    // Literal expression: 0x2a
    Hexadecimal(String),
    Radix(String),
    Operator(char),
    Assignment(char),
    Unknown(char),
    // Don't care token
    Skip,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum State {
    General,
    Front,
    Open,
    Close,
    // >> <<
    // DoubleOperator,
    // ^ | & ~ ) (
    // SingleOperator,
    Operator,
    // =
    Assignment,
    // Precedes an assignment
    // Variable,
    // Follows an assignment
    // Expression,
    // Hex, Bin, Dec
    Radix,
}

pub type Tokens = Vec<(usize, Token)>;

pub struct Lexer {
    pub tokens: Tokens,
    state: State,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            tokens: Tokens::with_capacity(10),
            state: State::Front,
        }
    }

    pub fn analyze(&mut self, line: &str) -> &Tokens {
        self.reset_lexer();

        let mut radix = String::with_capacity(2);
        for (position, character) in line.trim().chars().enumerate() {
            if character.is_whitespace() {
                continue;
            }

            println!("{}", character);
            let mut token = Token::Skip;
            match character {
                ')' => {
                    self.radix_check(position, &mut radix);
                    token = Token::CloseBracket;
                    self.state = State::Close;
                }
                '(' => {
                    self.radix_check(position, &mut radix);
                    token = Token::OpenBracket;
                    self.state = State::Open;
                }
                '^' | '&' | '|' | '~' => {
                    self.radix_check(position, &mut radix);
                    token = Token::Operator(character);
                    self.state = State::Operator;
                }
                '>' | '<' => {
                    self.radix_check(position, &mut radix);
                    token = Token::Operator(character);
                    self.state = State::Operator;
                }
                '=' => {
                    self.radix_check(position, &mut radix);
                    token = Token::Assignment(character);
                    self.state = State::Assignment;
                }
                _ if character.is_alphabetic() || character.is_numeric() => {
                    radix.push(character);
                    self.state = State::Radix;
                }
                _ => {
                    self.radix_check(position, &mut radix);
                    token = Token::Unknown(character);
                    self.state = State::General;
                }
            }

            if token != Token::Skip {
                self.tokens.push((position, token));
            }
        }
        if !radix.is_empty() {
            let mut token = Token::Radix(radix.clone());
            self.tokens.push((line.len() - radix.len(), token));
        }

        &self.tokens
    }

    fn radix_check(&mut self, position: usize, radix: &mut String) {
        if self.state == State::Radix {
            let mut token = Token::Radix(radix.clone());
            self.tokens.push((position - radix.len(), token));
            radix.clear();
        }

    }
    // TODO: Merge this with radix_check
    fn is_valid_number(&mut self, expression: &String) -> (bool, Token) {
        let mut token = Token::Skip;
        // Must be valid Binary, Hex, or Dec
        (false, token)
    }

    fn reset_lexer(&mut self) {
        self.state = State::Front;
        self.tokens.clear();
    }
}
