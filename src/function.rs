use std::collections::HashMap;
use error::EvaluatorError;
use lexer::Symbol;

type Operation = (i32, String);
pub type Functions = HashMap<Symbol, Function>;
pub type FunctionHandle = Box<Fn(Vec<i32>, usize) -> Result<Operation, EvaluatorError>>;

pub struct Function {
    pub arity: usize,
    pub handle: FunctionHandle,
}

impl Function {
    pub fn new(a: usize, h: FunctionHandle) -> Function {
        Function {
            arity: a,
            handle: h,
        }
    }
}

pub mod functions {
    use error::EvaluatorError;
    type Operation = (i32, String);

    pub fn not(mut args: Vec<i32>, _position: usize) -> Result<Operation, EvaluatorError> {
        let a: i32 = args.pop().unwrap();
        Ok((!a, format!("{}{}", "~".to_string(), a.to_string())))
    }

    pub fn and(mut args: Vec<i32>, _position: usize) -> Result<Operation, EvaluatorError> {
        let b = args.pop().unwrap();
        let a = args.pop().unwrap();
        Ok((a & b, format!("{} {} {}", a.to_string(), "&".to_string(), b.to_string())))
    }

    pub fn or(mut args: Vec<i32>, _position: usize) -> Result<Operation, EvaluatorError> {
        let b = args.pop().unwrap();
        let a = args.pop().unwrap();
        Ok((a | b, format!("{} {} {}", a.to_string(), "|".to_string(), b.to_string())))
    }

    pub fn xor(mut args: Vec<i32>, _position: usize) -> Result<Operation, EvaluatorError> {
        let b = args.pop().unwrap();
        let a = args.pop().unwrap();
        Ok((a ^ b, format!("{} {} {}", a.to_string(), "^".to_string(), b.to_string())))
    }

    pub fn rshift(mut args: Vec<i32>, _position: usize) -> Result<Operation, EvaluatorError> {
        let b = args.pop().unwrap();
        let a = args.pop().unwrap();
        if b < 0 {
            return Err(EvaluatorError::NegativeShift(_position));
        }
        Ok((a >> b, format!("{} {} {}", a.to_string(), ">>".to_string(), b.to_string())))
    }

    pub fn lshift(mut args: Vec<i32>, _position: usize) -> Result<Operation, EvaluatorError> {
        let b = args.pop().unwrap();
        let a = args.pop().unwrap();
        if b < 0 {
            return Err(EvaluatorError::NegativeShift(_position));
        }
        Ok((a << b, format!("{} {} {}", a.to_string(), "<<".to_string(), b.to_string())))
    }
}
