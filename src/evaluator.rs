use error::EvaluatorError;

pub struct Evaluator {

}

// impl Evaluator {
//     pub fn new() -> Evaluator {}
//
// fn is_keyword(&mut self, identifier: String) -> bool {
//     for keyword in KEYWORDS {
//         if keyword.to_string() == identifier.to_lowercase() {
//             return true;
//         }
//     }
//     return false;
// }


// pub fn evaluate() {}
// }

pub mod functions {
    pub fn not(mut a: i32) -> i32 {
        !a
    }
    pub fn and(mut a: i32, mut b: i32) -> i32 {
        a & b
    }
    pub fn or(mut a: i32, mut b: i32) -> i32 {
        a | b
    }
    pub fn xor(mut a: i32, mut b: i32) -> i32 {
        a ^ b
    }
    pub fn right_shift(mut a: i32, mut b: i32) -> i32 {
        a >> b
    }
    pub fn left_shift(mut a: i32, mut b: i32) -> i32 {
        a << b
    }
}
