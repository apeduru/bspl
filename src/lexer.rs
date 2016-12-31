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
    Radix(String), // TEMP: Remove when done classifying Hex, Bin, Dec, Var
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
    Operator,
    Assignment,
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
                    self.state = State::Close;
                }
                '(' => {
                    self.radix_check(start_position, &mut radix);
                    token = Token::OpenBracket;
                    start_position = position;
                    self.state = State::Open;
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
                _ if character.is_alphabetic() || character.is_numeric() => {
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
        if !radix.is_empty() {
            self.tokens.push((start_position, Token::Radix(radix.clone())));
        }

        &self.tokens
    }

    fn radix_check(&mut self, start_position: usize, radix: &mut String) {
        if self.state == State::Radix {
            self.tokens.push((start_position, Token::Radix(radix.clone())));
            radix.clear();
        }

    }

    // // TODO: Merge this with radix_check
    // fn is_valid_number(&mut self, expression: &String) -> (bool, Token) {
    //     let token = Token::Skip;
    //     // Must be valid Binary, Hex, or Dec
    //     (false, token)
    // }

    fn reset_lexer(&mut self) {
        self.state = State::Front;
        self.tokens.clear();
    }
}
