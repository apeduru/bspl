// #[derive(Debug, PartialEq)]
// pub enum LexerError {
//     // Can only use '(' or ')'
//     BracketError(usize),
//     // Can't assign to operator
//     SyntaxError(usize),
//     // Can't assign to keywords
//     KeywordError(String),
// }

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    IllegalOperator(usize, String),
    UnknownVariable(usize, String),
    MissingBracket(usize, String),
}

#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    BaseError,
}

#[derive(Debug, PartialEq)]
pub enum ConversionError {
    HexError,
    BinError,
    DecError,
}
