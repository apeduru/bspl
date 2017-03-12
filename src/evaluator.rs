use lexer::{Symbol, Token, Tokens};
use function::{Function, Functions, functions};
use error::EvaluatorError;
use constants::{VERSION, KEYWORDS, HELP, LICENSE};


pub struct Evaluator {
    functions: Functions,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator { functions: Functions::new() }
    }

    fn is_keyword(&self, variable: &String) -> Option<&'static str> {
        for keyword in KEYWORDS {
            if keyword.to_string() == variable.to_lowercase() {
                return Some(keyword);
            }
        }

        None
    }

    pub fn evaluate(&self, tokens: Tokens) -> Result<Vec<String>, EvaluatorError> {
        let mut result: Vec<String> = Vec::with_capacity(3);
        let mut stack: Vec<i32> = Vec::with_capacity(3);
        let num_tokens = tokens.len();

        if tokens.is_empty() {
            return Ok(result);
        }

        for (position, ref token) in tokens {
            match *token {
                Token::Decimal(ref dec) => {
                    stack.push(dec.parse().unwrap());
                }
                Token::Variable(ref var) => {
                    if let Some(kw) = self.is_keyword(var) {
                        if num_tokens == 1 {
                            match kw {
                                "version" => result.push(VERSION.to_string()),
                                "help" => {
                                    let mut h = HELP.lines()
                                        .map(|line| line.to_string())
                                        .collect();
                                    result.append(&mut h);
                                }
                                "license" => {
                                    let mut l = LICENSE.lines()
                                        .map(|line| line.to_string())
                                        .collect();
                                    result.append(&mut l);
                                }
                                "exit" => return Err(EvaluatorError::Exit),
                                _ => unreachable!(),
                            }
                            return Ok(result);
                        } else {
                            return Err(EvaluatorError::KeywordError(position));
                        }
                    } else {
                        return Err(EvaluatorError::UnknownKeyword(position));
                    }
                }
                Token::Operator(ref op) => {
                    let function = self.functions.get(&op).unwrap();
                    if stack.len() >= function.arity {
                        let stack_len = stack.len();
                        let args: Vec<i32> = stack.split_off(stack_len - function.arity);
                        let interm_result = try!((function.handle)(args, position));
                        stack.push(interm_result.0);
                        result.push(interm_result.1);
                        result.push(interm_result.0.to_string());
                    } else {
                        return Err(EvaluatorError::MissingArgument(position));
                    }
                }
                _ => continue,

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

        evaluator.functions.insert(Symbol::NOT, Function::new(1, Box::new(functions::not)));
        evaluator.functions.insert(Symbol::XOR, Function::new(2, Box::new(functions::xor)));
        evaluator.functions.insert(Symbol::OR, Function::new(2, Box::new(functions::or)));
        evaluator.functions.insert(Symbol::AND, Function::new(2, Box::new(functions::and)));
        evaluator.functions.insert(Symbol::LSHIFT,
                                   Function::new(2, Box::new(functions::lshift)));
        evaluator.functions.insert(Symbol::RSHIFT,
                                   Function::new(2, Box::new(functions::rshift)));

        evaluator
    }
}
