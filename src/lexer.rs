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
    Decimal(String), // Literal expression: 42
    Radix(String), // Non-Dec
    Operator(Symbol),
    Unknown(char),
    Skip, // Placeholder/Don't care token. Every character is tokenized.
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum State {
    General,
    Front,
    Bracket,
    Operator,
    Shift,
    Radix,
}

pub type Tokens = Vec<(usize, Token)>;

pub struct Lexer {
    curr_state: State,
    prev_state: State,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            curr_state: State::Front,
            prev_state: State::General,
        }
    }

    pub fn analyze(&mut self, line: &str) -> Tokens {
        let mut tokens = Tokens::new();

        let mut radix = String::with_capacity(2);
        let mut shift = String::with_capacity(2);
        let mut radix_position: usize = 0;
        let mut shift_position: usize = 0;

        for (position, character) in line.chars().enumerate() {
            if character.is_whitespace() {
                continue;
            }

            let mut token = Token::Skip;
            self.prev_state = self.curr_state;

            match character {
                ')' => {
                    token = Token::CloseBracket;
                    self.curr_state = State::Bracket;
                }
                '(' => {
                    token = Token::OpenBracket;
                    self.curr_state = State::Bracket;
                }
                '^' => {
                    token = Token::Operator(Symbol::XOR);
                    self.curr_state = State::Operator;
                }
                '&' => {
                    token = Token::Operator(Symbol::AND);
                    self.curr_state = State::Operator;
                }
                '|' => {
                    token = Token::Operator(Symbol::OR);
                    self.curr_state = State::Operator;
                }
                '~' => {
                    token = Token::Operator(Symbol::NOT);
                    self.curr_state = State::Operator;
                }
                '>' | '<' => {
                    if shift.is_empty() {
                        shift_position = position;
                    }
                    shift.push(character);
                    self.curr_state = State::Shift;
                }
                _ if character.is_alphanumeric() => {
                    if radix.is_empty() {
                        radix_position = position;
                    }
                    radix.push(character);
                    self.curr_state = State::Radix;
                }
                _ => {
                    token = Token::Unknown(character);
                    self.curr_state = State::General;
                }
            }

            if self.prev_state == State::Radix && self.curr_state != State::Radix {
                self.identify_radix(radix_position, &mut tokens, &mut radix);
            }

            if (shift.len() == 2 && self.curr_state == State::Shift) ||
               (shift.len() > 0 && self.curr_state != State::Shift) {
                self.identify_shift(shift_position, &mut tokens, &mut shift);
            }

            if token != Token::Skip {
                tokens.push((position, token.clone()));
            }

        }

        if !radix.is_empty() {
            self.identify_radix(radix_position, &mut tokens, &mut radix);
        }
        if !shift.is_empty() {
            self.identify_shift(shift_position, &mut tokens, &mut shift);
        }

        self.reset_lexer();

        tokens
    }

    fn identify_shift(&self, position: usize, tokens: &mut Tokens, shift: &mut String) {
        if shift.as_str() == "<<" {
            tokens.push((position, Token::Operator(Symbol::LSHIFT)));
        } else if shift.as_str() == ">>" {
            tokens.push((position, Token::Operator(Symbol::RSHIFT)));
        } else {
            let mut err_position = position;
            for ch in shift.chars() {
                tokens.push((err_position, Token::Unknown(ch)));
                err_position += 1;
            }
        }
        shift.clear();
    }

    fn identify_radix(&self, position: usize, tokens: &mut Tokens, radix: &mut String) {
        if !radix.parse::<i32>().is_err() {
            tokens.push((position, Token::Decimal(radix.clone())));
        } else {
            tokens.push((position, Token::Radix(radix.clone())));
        }
        radix.clear();
    }


    fn reset_lexer(&mut self) {
        self.curr_state = State::Front;
        self.prev_state = State::General;
    }
}
