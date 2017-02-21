use std::collections::HashMap;
use lexer::{Token, Tokens};
use error::ParserError;

type Operators = HashMap<&'static str, Operator>;

#[derive(PartialEq, Debug)]
enum Associativity {
    LeftToRight,
    RightToLeft,
}

#[derive(PartialEq, Debug)]
struct Operator(usize, Associativity);

impl Operator {
    fn new(precedence: usize, associativity: Associativity) -> Operator {
        Operator(precedence, associativity)
    }
}

pub struct Parser {
    operators: Operators,
    stack: Tokens,
    output: Tokens,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            operators: Operators::new(),
            stack: Tokens::with_capacity(3),
            output: Tokens::with_capacity(3),
        }
    }


    fn lower_precedence(&self, new_token: &Token, top_token: &Token) -> bool {
        let &Operator(new_token_prec, ref new_token_assoc) = match *new_token {
            Token::Operator(ref name) => self.operators.get::<str>(&name).unwrap(),
            _ => unreachable!(),
        };

        let &Operator(top_token_prec, _) = match *top_token {
            Token::Operator(ref name) => self.operators.get::<str>(&name).unwrap(),
            _ => unreachable!(),
        };

        (*new_token_assoc == Associativity::LeftToRight && new_token_prec <= top_token_prec) ||
        (*new_token_assoc == Associativity::RightToLeft && new_token_prec < top_token_prec)
    }

    pub fn parse(&mut self, tokens: &Tokens) -> Result<&Tokens, ParserError> {
        self.reset_parser();

        let mut iter = tokens.iter().peekable();
        while let Some(&(position, ref token)) = iter.next() {
            match *token {
                Token::Decimal(_) => self.output.push((position, token.clone())),

                // Token::Identifier(ref id) => {
                //     if self.is_keyword(id) {
                //         return Err(ParserError::KeywordError(position, id.clone()));
                //     } else {
                //         self.stack.push((position, token.clone()));
                //     }
                // }
                Token::Operator(ref name) => {
                    // if the token is an operator, o1, then:
                    // while there is an operator token o2, at the top of the
                    // operator stack and either o1 is left-associative and its
                    // precedence is less than or equal to that of o2, or o1 is
                    // right associative, and has precedence less than that of
                    // o2, pop o2 off the operator stack, onto the output queue
                    if !self.operators.contains_key::<str>(&name) {
                        return Err(ParserError::IllegalOperator(position));
                    }

                    loop {
                        match self.stack.last() {
                            Some(&(_, Token::Operator(_))) => {
                                if self.lower_precedence(&token, &self.stack.last().unwrap().1) {
                                    self.output.push(self.stack.pop().unwrap());
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }

                    self.stack.push((position, token.clone()));
                }
                Token::OpenBracket => self.stack.push((position, token.clone())),
                Token::CloseBracket => {
                    let mut found = false;

                    loop {
                        match self.stack.last() {
                            Some(&(_, Token::OpenBracket)) => {
                                found = true;
                                self.stack.pop();
                                break;
                            }
                            None => break,
                            _ => self.output.push(self.stack.pop().unwrap()),
                        }
                    }
                    if !found {
                        return Err(ParserError::MissingOpeningBracket(position));
                    }
                }
                Token::Unknown(_) => {
                    return Err(ParserError::IllegalOperator(position));
                }
                _ => continue,

            }
        }
        loop {
            match self.stack.last() {
                Some(&(position, Token::OpenBracket)) => {
                    return Err(ParserError::MissingClosingBracket(position))
                }
                Some(_) => self.output.push(self.stack.pop().unwrap()),
                None => break,
            }
        }

        Ok((&self.output))
    }

    fn reset_parser(&mut self) {
        self.stack.clear();
        self.output.clear();
    }
}

impl Default for Parser {
    fn default() -> Parser {
        let mut parser = Parser::new();

        parser.operators.insert("(", Operator::new(1, Associativity::LeftToRight));
        parser.operators.insert(")", Operator::new(1, Associativity::LeftToRight));
        parser.operators.insert("~", Operator::new(2, Associativity::RightToLeft));
        parser.operators.insert(">>", Operator::new(3, Associativity::LeftToRight));
        parser.operators.insert("<<", Operator::new(3, Associativity::LeftToRight));
        parser.operators.insert("&", Operator::new(4, Associativity::LeftToRight));
        parser.operators.insert("^", Operator::new(5, Associativity::LeftToRight));
        parser.operators.insert("|", Operator::new(6, Associativity::LeftToRight));

        parser
    }
}
