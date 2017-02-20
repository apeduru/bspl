use std::collections::HashMap;
use error::EvaluatorError;

pub type Functions = HashMap<&'static str, Function>;
pub type Operation = (i32, String);
pub type FunctionHandle = Box<Fn(Vec<i32>) -> Result<Operation, EvaluatorError>>;

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
    pub type Operation = (i32, String);
    use error::EvaluatorError;

    // pub fn minus(mut args: Vec<i32>) -> Result<i32, EvaluatorError> {
    //     Ok(-args.pop().unwrap())
    // }

    pub fn not(mut args: Vec<i32>) -> Result<Operation, EvaluatorError> {
        let mut a: i32 = args.pop().unwrap();
        Ok((!a, format!("{}{}", "~".to_string(), a.to_string())))
    }

    pub fn and(mut args: Vec<i32>) -> Result<Operation, EvaluatorError> {
        let mut b = args.pop().unwrap();
        let mut a = args.pop().unwrap();
        Ok((a & b, format!("{} {} {}", a.to_string(), "&".to_string(), b.to_string())))
    }

    pub fn or(mut args: Vec<i32>) -> Result<Operation, EvaluatorError> {
        let mut b = args.pop().unwrap();
        let mut a = args.pop().unwrap();
        Ok((a | b, format!("{} {} {}", a.to_string(), "|".to_string(), b.to_string())))
    }

    pub fn xor(mut args: Vec<i32>) -> Result<Operation, EvaluatorError> {
        let mut b = args.pop().unwrap();
        let mut a = args.pop().unwrap();
        Ok((a ^ b, format!("{} {} {}", a.to_string(), "^".to_string(), b.to_string())))
    }

    pub fn rshift(mut args: Vec<i32>) -> Result<Operation, EvaluatorError> {
        let mut b = args.pop().unwrap();
        let mut a = args.pop().unwrap();
        Ok((a >> b, format!("{} {} {}", a.to_string(), ">>".to_string(), b.to_string())))
    }

    pub fn lshift(mut args: Vec<i32>) -> Result<Operation, EvaluatorError> {
        let mut b = args.pop().unwrap();
        let mut a = args.pop().unwrap();
        Ok((a << b, format!("{} {} {}", a.to_string(), "<<".to_string(), b.to_string())))
    }

}
