use lexer::{Symbol, Token, Tokens};
use function::{functions, Function, Functions};
use error::EvaluatorError;
use constants::{HELP, KEYWORDS, LICENSE, VERSION};
use std::u32;

pub struct Evaluator {
    functions: Functions,
}

fn is_keyword(variable: &String) -> Option<&str> {
    for keyword in KEYWORDS {
        if keyword.to_string() == variable.to_lowercase() {
            return Some(keyword);
        }
    }

    None
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            functions: Functions::new(),
        }
    }

    pub fn evaluate(&self, tokens: Tokens) -> Result<Vec<String>, EvaluatorError> {
        let mut result: Vec<String> = Vec::new();
        let mut stack: Vec<u32> = Vec::new();

        if tokens.is_empty() {
            return Ok(result);
        }

        for (position, token) in tokens {
            match token {
                Token::Decimal(ref dec) => {
                    stack.push(dec.parse().unwrap());
                }
                Token::Hexadecimal(ref hex) => {
                    stack.push(u32::from_str_radix(hex.as_str().split_at(2).1, 16).unwrap());
                }
                Token::Keyword(ref kw) => {
                    if let Some(keyword) = is_keyword(kw) {
                        match keyword {
                            "version" => result.push(VERSION.to_string()),
                            "help" => {
                                result = HELP.lines().map(|line| line.to_string()).collect();
                            }
                            "license" => {
                                result = LICENSE.lines().map(|line| line.to_string()).collect();
                            }
                            "exit" => return Err(EvaluatorError::Exit),
                            _ => unreachable!(),
                        }
                        return Ok(result);
                    } else {
                        return Err(EvaluatorError::UnknownKeyword(position));
                    }
                }
                Token::Operator(ref op) => {
                    let function = self.functions.get(&op).unwrap();
                    if stack.len() < function.arity {
                        return Err(EvaluatorError::MissingArgument(position));
                    }
                    let stack_len = stack.len();
                    let args: Vec<u32> = stack.split_off(stack_len - function.arity);
                    let interm_result = try!((function.handle)(args, position));
                    stack.push(interm_result.0);
                    result.push(interm_result.1);
                    result.push(interm_result.0.to_string());
                }
                _ => unreachable!(),
            }
        }

        // The last element on the stack should be the final result
        if stack.len() != 1 {
            return Err(EvaluatorError::TooManyArguments);
        } else if result.is_empty() {
            result.push(stack.pop().unwrap().to_string());
        }

        Ok(result)
    }
}

impl Default for Evaluator {
    fn default() -> Evaluator {
        let mut evaluator = Evaluator::new();

        evaluator.functions.insert(
            Symbol::NOT,
            Function::new(1, Box::new(functions::not))
        );
        evaluator.functions.insert(
            Symbol::XOR,
            Function::new(2, Box::new(functions::xor))
        );
        evaluator.functions.insert(
            Symbol::OR,
            Function::new(2, Box::new(functions::or))
        );
        evaluator.functions.insert(
            Symbol::AND,
            Function::new(2, Box::new(functions::and))
        );
        evaluator.functions.insert(
            Symbol::LSHIFT,
            Function::new(2, Box::new(functions::lshift))
        );
        evaluator.functions.insert(
            Symbol::RSHIFT,
            Function::new(2, Box::new(functions::rshift))
        );

        evaluator
    }
}

#[cfg(test)]
mod tests {
    use lexer::{Symbol, Token, Tokens};
    use evaluator::Evaluator;
    use error::EvaluatorError;
    use constants::{HELP, LICENSE};

    #[test]
    fn blank() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![];
        let result: Vec<String> = vec![];
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn decimal() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![(0, Token::Decimal("12".to_string()))];
        let result: Vec<String> = vec!["12".to_string()];
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn hexadecimal() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![(0, Token::Hexadecimal("0xc".to_string()))];
        let result: Vec<String> = vec!["12".to_string()];
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn keyword_exit() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![(0, Token::Keyword("exit".to_string()))];
        assert_eq!(evaluator.evaluate(tokens), Err(EvaluatorError::Exit));
    }

    #[test]
    fn keyword_license() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![(0, Token::Keyword("license".to_string()))];
        let result: Vec<String> = LICENSE.lines().map(|line| line.to_string()).collect();
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn keyword_help() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![(0, Token::Keyword("help".to_string()))];
        let result: Vec<String> = HELP.lines().map(|line| line.to_string()).collect();
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn keyword_version() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![(0, Token::Keyword("version".to_string()))];
        let result: Vec<String> = vec![env!("CARGO_PKG_VERSION").to_string()];
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn keyword_unknown() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![(0, Token::Keyword("rust".to_string()))];
        assert_eq!(
            evaluator.evaluate(tokens),
            Err(EvaluatorError::UnknownKeyword(0))
        );
    }

    #[test]
    fn expression_hexadecimal() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![
            (0, Token::Hexadecimal("0x1".to_string())),
            (4, Token::Hexadecimal("0xc".to_string())),
            (2, Token::Operator(Symbol::LSHIFT)),
        ];
        let result: Vec<String> = vec!["1 << 12".to_string(), "4096".to_string()];
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn expression_decimal_lshift() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("1".to_string())),
            (4, Token::Decimal("12".to_string())),
            (2, Token::Operator(Symbol::LSHIFT)),
        ];
        let result: Vec<String> = vec!["1 << 12".to_string(), "4096".to_string()];
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn expression_decimal_lshift_overflow() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("4294967295".to_string())),
            (14, Token::Decimal("32".to_string())),
            (11, Token::Operator(Symbol::LSHIFT)),
        ];
        assert_eq!(
            evaluator.evaluate(tokens),
            Err(EvaluatorError::OverflowShift(11))
        );
    }

    #[test]
    fn expression_decimal_rshift() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("12".to_string())),
            (4, Token::Decimal("1".to_string())),
            (2, Token::Operator(Symbol::RSHIFT)),
        ];
        let result: Vec<String> = vec!["12 >> 1".to_string(), "6".to_string()];
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn expression_decimal_rshift_overflow() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("1".to_string())),
            (4, Token::Decimal("32".to_string())),
            (2, Token::Operator(Symbol::RSHIFT)),
        ];
        assert_eq!(
            evaluator.evaluate(tokens),
            Err(EvaluatorError::OverflowShift(2))
        );
    }

    #[test]
    fn expression_decimal_xor() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("1".to_string())),
            (4, Token::Decimal("12".to_string())),
            (2, Token::Operator(Symbol::XOR)),
        ];
        let result: Vec<String> = vec!["1 ^ 12".to_string(), "13".to_string()];
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn expression_decimal_and() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("1".to_string())),
            (4, Token::Decimal("12".to_string())),
            (2, Token::Operator(Symbol::AND)),
        ];
        let result: Vec<String> = vec!["1 & 12".to_string(), "0".to_string()];
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn expression_decimal_or() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("32".to_string())),
            (5, Token::Decimal("10".to_string())),
            (3, Token::Operator(Symbol::OR)),
        ];
        let result: Vec<String> = vec!["32 | 10".to_string(), "42".to_string()];
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn expression_decimal_not() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![
            (1, Token::Decimal("12".to_string())),
            (0, Token::Operator(Symbol::NOT)),
        ];
        let result: Vec<String> = vec!["~12".to_string(), "4294967283".to_string()];
        assert_eq!(evaluator.evaluate(tokens).unwrap(), result);
    }

    #[test]
    fn expression_missing_argument() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("1".to_string())),
            (2, Token::Operator(Symbol::AND)),
        ];
        assert_eq!(
            evaluator.evaluate(tokens),
            Err(EvaluatorError::MissingArgument(2))
        );
    }

    #[test]
    fn expression_missing_arguments() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![(0, Token::Operator(Symbol::AND))];
        assert_eq!(
            evaluator.evaluate(tokens),
            Err(EvaluatorError::MissingArgument(0))
        );
    }

    #[test]
    fn expression_too_many_arguments() {
        let evaluator = Evaluator::default();
        let tokens: Tokens = vec![
            (0, Token::Decimal("1".to_string())),
            (6, Token::Decimal("12".to_string())),
            (11, Token::Hexadecimal("0xf".to_string())),
            (8, Token::Operator(Symbol::LSHIFT)),
        ];
        assert_eq!(
            evaluator.evaluate(tokens),
            Err(EvaluatorError::TooManyArguments)
        );
    }

}
