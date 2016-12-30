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
    DoubleOperator(char),
    SingleOperator(char),
    Assignment(char),
    Unknown(char),
    // Don't care token
    Skip,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum State {
    General,
    Front,
    // >> <<
    // DoubleOperator,
    // ^ | & ~ ) (
    // SingleOperator,
    Operator,
    // =
    Assignment,
    // Precedes an assignment
    Variable,
    // Follows an assignment
    Expression,
    // Hex, Bin, Dec
    Radix,
}

pub type Tokens = Vec<(usize, Token)>;
// pub type Variables = HashMap<String, String>;

pub struct Lexer {
    pub tokens: Tokens,
    // pub variables: Variables,
    curr_state: State,
    prev_state: State,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            tokens: Tokens::with_capacity(10),
            // variables: Variables::new(),
            curr_state: State::Front,
            prev_state: State::General,
        }
    }

    pub fn analyze(&mut self, line: &str) -> &Tokens {
        self.reset_lexer();

        let mut variable = String::with_capacity(1);
        let mut expression = String::with_capacity(2);
        let mut radix = String::with_capacity(1);

        for (position, character) in line.trim().chars().enumerate() {
            if character.is_whitespace() {
                continue;
            }
            println!("{}", character);

            let mut token = Token::Skip;
            self.prev_state = self.curr_state;

            match character {
                ')' => {
                    token = Token::OpenBracket;
                    self.curr_state = State::Front;
                }
                '(' => {
                    token = Token::CloseBracket;
                    self.curr_state = State::General;
                }
                '^' | '&' | '|' | '~' => {
                    token = Token::SingleOperator(character);
                    self.curr_state = State::Operator;
                }
                '>' | '<' => {
                    token = Token::DoubleOperator(character);
                    self.curr_state = State::Operator;
                }
                '=' => {
                    if self.curr_state == State::Variable {
                        token = Token::Variable(variable.clone());
                        self.tokens.push((0, token));
                    }
                    token = Token::Assignment(character);
                    self.curr_state = State::Assignment;
                }
                _ if character.is_alphabetic() => {
                    if self.curr_state == State::Front || self.curr_state == State::Variable {
                        variable.push(character);
                        self.curr_state = State::Variable;
                    } else if self.curr_state == State::Radix ||
                              self.curr_state == State::General ||
                              self.curr_state == State::Operator {
                        radix.push(character);
                        self.curr_state = State::Radix;
                    }
                }
                _ if character.is_numeric() => {
                    if self.curr_state == State::Assignment ||
                       self.curr_state == State::Expression {
                        expression.push(character);
                        self.curr_state = State::Expression;
                    } else if self.curr_state == State::Radix ||
                              self.curr_state == State::General ||
                              self.curr_state == State::Operator {
                        radix.push(character);
                        self.curr_state = State::Radix;
                    }
                }
                _ => {
                    token = Token::Unknown(character);
                    self.curr_state = State::General;
                }
            }

            if self.curr_state != State::Radix && !radix.is_empty() {
                // self.is_valid_expression(&radix);

            } else if !variable.is_empty() && !expression.is_empty() &&
                // HACK: Find a better way to find out if we've reached the end of the line
                      position == line.trim().len() - 1 {
                // let self.is_valid_expression(&expression)
                token = Token::Expression(expression.clone());
                // self.variables.insert(variable, expression);
                // FIXME: position of expression should NOT be line length, ideally position of
                // assignment + 1
                self.tokens.push((line.len(), token));
            } else if token != Token::Skip {
                self.tokens.push((position, token));
            }
        }
        return &self.tokens;
    }


    // TODO: Decide what to do with this validation function
    #[inline]
    fn is_valid_expression(&mut self, expression: &String) -> (bool, Token) {
        let mut token = Token::Skip;
        // Must be valid Binary, Hex, or Dec
        (false, token)
    }

    #[inline]
    fn reset_lexer(&mut self) {
        self.curr_state = State::Front;
        self.prev_state = State::General;
        self.tokens.clear();
    }
}
