// use std::collections::HashMap;
use lexer::{Symbol, Token, Tokens};
// use constants::KEYWORDS;
use function::{Function, Functions, functions};
use error::EvaluatorError;

// type Identifiers = HashMap<String, String>;

pub struct Evaluator {
    // identifiers: Identifiers,
    functions: Functions,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            // identifiers: Identifiers::new(),
            functions: Functions::new(),
        }
    }

    // fn is_keyword(&mut self, identifier: &String) -> bool {
    //     for keyword in KEYWORDS {
    //         if keyword.to_string() == identifier.to_lowercase() {
    //             return true;
    //         }
    //     }
    //     return false;
    // }

    pub fn evaluate(&mut self, tokens: Tokens) -> Result<Vec<String>, EvaluatorError> {
        let mut result: Vec<String> = Vec::with_capacity(3);
        let mut stack: Vec<i32> = Vec::with_capacity(3);

        for (position, ref token) in tokens {
            match *token {
                Token::Decimal(ref dec) => {
                    stack.push(dec.parse().unwrap());
                }
                Token::Operator(ref op) => {
                    let function = self.functions.get(&op).unwrap();
                    if stack.len() >= function.arity {
                        let stack_len = stack.len();
                        let args: Vec<i32> = stack.split_off(stack_len - function.arity);
                        let interm_result = (function.handle)(args).unwrap();
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
