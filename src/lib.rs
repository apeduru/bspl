pub mod keywords;
pub mod tokenizer;
pub mod parser;

pub type Variable = Vec<(char, f32)>;

pub enum Token {
    OpenBracket,
    CloseBracket,
    Variable(Variable),
    Operator(char),
    Unknown(char),
}

pub enum InputError {
    BracketError,
}

pub enum ParsingError {
    IllegalOperator(usize, char),
    MissingBracket(usize),
    UnknownVariable(usize, char),
}

pub enum EvaluationError {
    BaseError,
}

pub type TokenList = Vec<(usize, Token)>;
