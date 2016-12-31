#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    // Identifier, must strictly be alphabetical
    Identifier(String),
    // Literal expression: 42
    Decimal(String),
    // Literal expression: 0b101010
    Binary(String),
    // Literal expression: 0x2a
    Hexadecimal(String),
    // Invalid Radix
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
    Operator,
    Assignment,
    Identifier,
    Radix,
}

pub type Tokens = Vec<(usize, Token)>;

pub struct Lexer {
    tokens: Tokens,
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
        let mut start_position: usize = 0;
        for (position, character) in line.trim().chars().enumerate() {
            if character.is_whitespace() {
                continue;
            }

            let mut token = Token::Skip;
            match character {
                ')' => {
                    self.radix_check(start_position, &mut radix);
                    token = Token::CloseBracket;
                    start_position = position;
                    self.state = State::General;
                }
                '(' => {
                    self.radix_check(start_position, &mut radix);
                    token = Token::OpenBracket;
                    start_position = position;
                    self.state = State::Front;
                }
                '>' | '<' | '^' | '&' | '|' | '~' => {
                    self.radix_check(start_position, &mut radix);
                    token = Token::Operator(character);
                    start_position = position;
                    self.state = State::Operator;
                }
                '=' => {
                    self.radix_check(start_position, &mut radix);
                    token = Token::Assignment(character);
                    start_position = position;
                    self.state = State::Assignment;
                }
                _ if character.is_alphabetic() => {
                    if radix.is_empty() {
                        start_position = position;
                    }
                    radix.push(character);
                    if self.state != State::Radix {
                        self.state = State::Identifier;
                    }

                }
                // Any Radix is gauranteed to have at least 1 numeric character
                _ if character.is_numeric() => {
                    if radix.is_empty() {
                        start_position = position;
                    }
                    radix.push(character);
                    self.state = State::Radix;
                }
                _ => {
                    self.radix_check(start_position, &mut radix);
                    token = Token::Unknown(character);
                    start_position = position;
                    self.state = State::General;
                }
            }
            if token != Token::Skip {
                self.tokens.push((start_position, token));
            }
        }
        // Radix may occur at the end
        if !radix.is_empty() {
            let mut token = self.identify_radix(&mut radix);
            self.tokens.push((start_position, token));
        }

        &self.tokens
    }

    fn identify_radix(&mut self, radix: &mut String) -> Token {
        let mut token;
        if self.state == State::Radix {
            if radix.len() > 2 && radix.starts_with("0x") {
                token = Token::Hexadecimal(radix.clone());
            } else if radix.len() > 2 && radix.starts_with("0b") {
                token = Token::Binary(radix.clone());
            } else if !radix.starts_with("0b") && !radix.starts_with("0x") {
                token = Token::Decimal(radix.clone());
            } else {
                token = Token::Radix(radix.clone());
            }
        } else {
            token = Token::Identifier(radix.clone());
        }

        token
    }

    fn radix_check(&mut self, start_position: usize, radix: &mut String) {
        let mut token = self.identify_radix(radix);
        self.tokens.push((start_position, token));
        radix.clear();
    }

    fn reset_lexer(&mut self) {
        self.state = State::Front;
        self.tokens.clear();
    }
}
