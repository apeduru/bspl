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
        Parser {
            operators: Operators::new(),
        }
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

        (*new_token_assoc == Associativity::LeftToRight && new_token_prec > top_token_prec)
            || (*new_token_assoc == Associativity::RightToLeft && new_token_prec >= top_token_prec)
    }

    pub fn parse(&self, tokens: Tokens) -> Result<Tokens, ParserError> {
        let mut stack = Tokens::new();
        let mut output = Tokens::new();

        let mut token_iterator = tokens.iter().peekable();
        while let Some(&(position, ref token)) = token_iterator.next() {
            match *token {
                Token::Decimal(_) | Token::Hexadecimal(_) => output.push((position, token.clone())),
                Token::Keyword(_) => {
                    if tokens.len() > 1 {
                        return Err(ParserError::KeywordError(position));
                    }
                    output.push((position, token.clone()));
                    break;
                }
                Token::Operator(_) => {
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
                }
                Token::CloseBracket => loop {
                    match stack.last() {
                        Some(&(_, Token::OpenBracket)) => {
                            stack.pop();
                            break;
                        }
                        Some(_) => output.push(stack.pop().unwrap()),
                        None => return Err(ParserError::MissingOpeningBracket(position)),
                    }
                },
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

        parser.operators.insert(
            Symbol::NOT,
            Operator::new(2, Associativity::RightToLeft)
        );
        parser.operators.insert(
            Symbol::RSHIFT,
            Operator::new(3, Associativity::LeftToRight)
        );
        parser.operators.insert(
            Symbol::LSHIFT,
            Operator::new(3, Associativity::LeftToRight)
        );
        parser.operators.insert(
            Symbol::AND,
            Operator::new(4, Associativity::LeftToRight)
        );
        parser.operators.insert(
            Symbol::XOR,
            Operator::new(5, Associativity::LeftToRight)
        );
        parser.operators.insert(
            Symbol::OR,
            Operator::new(6, Associativity::LeftToRight)
        );

        parser
    }
}

#[cfg(test)]
mod tests {
    use lexer::{Symbol, Token, Tokens};
    use parser::Parser;
    use error::ParserError;

    #[test]
    fn blank() {
        let parser = Parser::default();
        let tokens: Tokens = vec![];
        let parsed: Tokens = vec![];
        assert_eq!(parser.parse(tokens).unwrap(), parsed);
    }

    #[test]
    fn expression_valid_decimal() {
        let parser = Parser::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("12".to_string())),
            (3, Token::Operator(Symbol::OR)),
            (5, Token::OpenBracket),
            (6, Token::Decimal("1".to_string())),
            (8, Token::Operator(Symbol::LSHIFT)),
            (11, Token::Decimal("12".to_string())),
            (13, Token::CloseBracket),
        ];
        let parsed: Tokens = vec![
            (0, Token::Decimal("12".to_string())),
            (6, Token::Decimal("1".to_string())),
            (11, Token::Decimal("12".to_string())),
            (8, Token::Operator(Symbol::LSHIFT)),
            (3, Token::Operator(Symbol::OR)),
        ];
        assert_eq!(parser.parse(tokens).unwrap(), parsed);
    }

    #[test]
    fn expression_valid_decimal_precedence() {
        let parser = Parser::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("12".to_string())),
            (3, Token::Operator(Symbol::XOR)),
            (6, Token::Decimal("3".to_string())),
            (8, Token::Operator(Symbol::OR)),
            (11, Token::Decimal("3".to_string())),
        ];

        let parsed: Tokens = vec![
            (0, Token::Decimal("12".to_string())),
            (6, Token::Decimal("3".to_string())),
            (3, Token::Operator(Symbol::XOR)),
            (11, Token::Decimal("3".to_string())),
            (8, Token::Operator(Symbol::OR)),
        ];

        assert_eq!(parser.parse(tokens).unwrap(), parsed);
    }

    #[test]
    fn expression_valid_hexadecimal() {
        let parser = Parser::default();
        let tokens: Tokens = vec![
            (0, Token::Hexadecimal("0xc".to_string())),
            (3, Token::Operator(Symbol::OR)),
            (5, Token::OpenBracket),
            (6, Token::Hexadecimal("0x1".to_string())),
            (8, Token::Operator(Symbol::LSHIFT)),
            (11, Token::Hexadecimal("0xc".to_string())),
            (13, Token::CloseBracket),
        ];
        let parsed: Tokens = vec![
            (0, Token::Hexadecimal("0xc".to_string())),
            (6, Token::Hexadecimal("0x1".to_string())),
            (11, Token::Hexadecimal("0xc".to_string())),
            (8, Token::Operator(Symbol::LSHIFT)),
            (3, Token::Operator(Symbol::OR)),
        ];
        assert_eq!(parser.parse(tokens).unwrap(), parsed);
    }

    #[test]
    fn expression_too_many_arguments() {
        let parser = Parser::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("1".to_string())),
            (8, Token::Operator(Symbol::LSHIFT)),
            (6, Token::Decimal("12".to_string())),
            (11, Token::Hexadecimal("0xf".to_string())),
        ];
        let parsed: Tokens = vec![
            (0, Token::Decimal("1".to_string())),
            (6, Token::Decimal("12".to_string())),
            (11, Token::Hexadecimal("0xf".to_string())),
            (8, Token::Operator(Symbol::LSHIFT)),
        ];
        assert_eq!(parser.parse(tokens).unwrap(), parsed);
    }

    #[test]
    fn keyword_valid() {
        let parser = Parser::default();
        let tokens: Tokens = vec![(0, Token::Keyword("exit".to_string()))];
        let parsed: Tokens = vec![(0, Token::Keyword("exit".to_string()))];
        assert_eq!(parser.parse(tokens).unwrap(), parsed);
    }

    #[test]
    fn expression_missing_open_bracket() {
        let parser = Parser::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("12".to_string())),
            (3, Token::Operator(Symbol::OR)),
            (6, Token::Decimal("1".to_string())),
            (8, Token::Operator(Symbol::LSHIFT)),
            (11, Token::Decimal("12".to_string())),
            (13, Token::CloseBracket),
        ];
        assert_eq!(
            parser.parse(tokens),
            Err(ParserError::MissingOpeningBracket(13))
        );
    }

    #[test]
    fn expression_missing_close_bracket() {
        let parser = Parser::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("12".to_string())),
            (3, Token::Operator(Symbol::OR)),
            (5, Token::OpenBracket),
            (6, Token::Decimal("1".to_string())),
            (8, Token::Operator(Symbol::LSHIFT)),
            (11, Token::Decimal("12".to_string())),
        ];
        assert_eq!(
            parser.parse(tokens),
            Err(ParserError::MissingClosingBracket(5))
        );
    }

    #[test]
    fn keyword_too_many() {
        let parser = Parser::default();
        let tokens: Tokens = vec![
            (0, Token::Keyword("exit".to_string())),
            (5, Token::Keyword("help".to_string())),
        ];
        assert_eq!(parser.parse(tokens), Err(ParserError::KeywordError(0)));
    }

    #[test]
    fn keyword_in_expression() {
        let parser = Parser::default();
        let tokens: Tokens = vec![
            (0, Token::Keyword("exit".to_string())),
            (5, Token::Operator(Symbol::OR)),
            (7, Token::Keyword("help".to_string())),
        ];
        assert_eq!(parser.parse(tokens), Err(ParserError::KeywordError(0)));
    }

}
