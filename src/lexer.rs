#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    Identifier(String), // Identifier, must be alphabetical
    Decimal(String), // Literal expression: 42
    Binary(String), // Literal expression: 0b101010
    // Octal(String), // Literal expression: 0o052
    Hexadecimal(String), // Literal expression: 0x2a
    Radix(String), // Non-{Bin, Hex, Dec, Oct}
    Operator(String),
    // Assignment(String),
    Unknown(char),
    // Placeholder/Don't care token. Every character is tokenized.
    Skip,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum State {
    General,
    Front,
    Bracket,
    Operator,
    Shift,
    // Assignment,
    Identifier,
    Radix,
}

pub type Tokens = Vec<(usize, Token)>;

pub struct Lexer {
    tokens: Tokens,
    curr_state: State,
    prev_state: State,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            tokens: Tokens::with_capacity(10),
            curr_state: State::Front,
            prev_state: State::General,
        }
    }

    pub fn analyze(&mut self, line: &str) -> &Tokens {
        self.reset_lexer();

        let mut radix = String::with_capacity(2);
        let mut shift = String::with_capacity(2);
        let mut start_position: usize = 0;
        let mut radix_position: usize = 0;
        let mut shift_position: usize = 0;

        for (position, character) in line.trim().chars().enumerate() {
            if character.is_whitespace() {
                continue;
            }

            let mut token = Token::Skip;
            self.prev_state = self.curr_state;
            match character {
                ')' => {
                    token = Token::CloseBracket;
                    start_position = position;
                    self.curr_state = State::Bracket;
                }
                '(' => {
                    token = Token::OpenBracket;
                    start_position = position;
                    self.curr_state = State::Bracket;
                }
                '^' | '&' | '|' | '~' | '-' => {
                    token = Token::Operator(character.to_string());
                    start_position = position;
                    self.curr_state = State::Operator;
                }
                '>' | '<' => {
                    if shift.is_empty() {
                        shift_position = position;
                    }
                    shift.push(character);
                    self.curr_state = State::Shift;
                }
                _ if character.is_alphabetic() => {
                    if radix.is_empty() {
                        radix_position = position;
                    }
                    radix.push(character);
                    if self.curr_state != State::Radix {
                        self.curr_state = State::Identifier;
                    }

                }
                _ if character.is_numeric() => {
                    if radix.is_empty() {
                        radix_position = position;
                    }
                    radix.push(character);
                    self.curr_state = State::Radix;
                }
                _ => {
                    token = Token::Unknown(character);
                    start_position = position;
                    self.curr_state = State::General;
                }
            }

            if (self.prev_state == State::Radix && self.curr_state != State::Radix) ||
               (self.prev_state == State::Identifier && self.curr_state != State::Identifier) {
                self.identify_radix(radix_position, &mut radix);
            }

            if self.prev_state == State::Shift && self.curr_state != State::Shift {
                self.tokens.push((shift_position, Token::Operator(shift.clone())));
                shift.clear();
            }

            if token != Token::Skip {
                self.tokens.push((start_position, token.clone()));
            }


        }

        if !radix.is_empty() {
            self.identify_radix(radix_position, &mut radix);
        }

        if !shift.is_empty() {
            self.tokens.push((shift_position, Token::Operator(shift.clone())));
            shift.clear();
        }

        &self.tokens
    }

    fn identify_radix(&mut self, position: usize, radix: &mut String) {
        if !radix.parse::<i32>().is_err() {
            self.tokens.push((position, Token::Decimal(radix.clone())));
        } else {
            self.tokens.push((position, Token::Radix(radix.clone())));
        }
        radix.clear();
    }


    fn reset_lexer(&mut self) {
        self.curr_state = State::Front;
        self.prev_state = State::General;
        self.tokens.clear();
    }
}


// #[cfg(test)]
// mod tests {}
