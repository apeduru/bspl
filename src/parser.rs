use std::collections::HashMap;
use lexer::{Symbol, Token, Tokens};
use error::ParserError;

type Operators = HashMap<Symbol, Operator>;

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
}

impl Parser {
    fn new() -> Parser {
        Parser { operators: Operators::new() }
    }

    fn lower_precedence(&self, new_token: &Token, top_token: &Token) -> bool {
        let &Operator(new_token_prec, ref new_token_assoc) = match *new_token {
            Token::Operator(ref new_token_name) => self.operators.get(&new_token_name).unwrap(),
            _ => unreachable!(),
        };

        let &Operator(top_token_prec, _) = match *top_token {
            Token::Operator(ref top_token_name) => self.operators.get(&top_token_name).unwrap(),
            _ => unreachable!(),
        };

        (*new_token_assoc == Associativity::LeftToRight && new_token_prec > top_token_prec) ||
        (*new_token_assoc == Associativity::RightToLeft && new_token_prec >= top_token_prec)
    }

    pub fn parse(&mut self, tokens: Tokens) -> Result<Tokens, ParserError> {
        let mut stack = Tokens::new();
        let mut output = Tokens::new();

        let mut token_iterator = tokens.iter().peekable();
        while let Some(&(position, ref token)) = token_iterator.next() {
            match *token {
                Token::Decimal(_) |
                Token::Variable(_) => output.push((position, token.clone())),
                Token::Radix(_) => {
                    return Err(ParserError::RadixError(position));
                }
                Token::Operator(ref name) => {
                    if !self.operators.contains_key(&name) {
                        return Err(ParserError::UnknownOperator(position));
                    }

                    loop {
                        match stack.last() {
                            Some(&(_, Token::Operator(_))) => {
                                if self.lower_precedence(&token, &stack.last().unwrap().1) {
                                    output.push(stack.pop().unwrap());
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }

                    stack.push((position, token.clone()));
                }
                Token::OpenBracket => {
                    stack.push((position, token.clone()));
                    if let Some(&&(position, Token::Operator(ref op))) = token_iterator.peek() {
                        if *op != Symbol::NOT {
                            return Err(ParserError::InvalidSyntax(position));
                        }
                    }

                }
                Token::CloseBracket => {
                    loop {
                        match stack.last() {
                            Some(&(_, Token::OpenBracket)) => {
                                stack.pop();
                                break;
                            }
                            None => return Err(ParserError::MissingOpeningBracket(position)),
                            _ => output.push(stack.pop().unwrap()),
                        }
                    }
                }
                Token::Unknown(_) => {
                    return Err(ParserError::UnknownOperator(position));
                }
                _ => continue,

            }
        }
        loop {
            match stack.last() {
                Some(&(position, Token::OpenBracket)) => {
                    return Err(ParserError::MissingClosingBracket(position))
                }
                Some(_) => output.push(stack.pop().unwrap()),
                None => break,
            }
        }

        Ok(output)
    }
}

impl Default for Parser {
    fn default() -> Parser {
        let mut parser = Parser::new();

        parser.operators.insert(Symbol::NOT, Operator::new(2, Associativity::RightToLeft));
        parser.operators.insert(Symbol::RSHIFT, Operator::new(3, Associativity::LeftToRight));
        parser.operators.insert(Symbol::LSHIFT, Operator::new(3, Associativity::LeftToRight));
        parser.operators.insert(Symbol::AND, Operator::new(4, Associativity::LeftToRight));
        parser.operators.insert(Symbol::XOR, Operator::new(5, Associativity::LeftToRight));
        parser.operators.insert(Symbol::OR, Operator::new(6, Associativity::LeftToRight));

        parser
    }
}
