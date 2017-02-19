#[derive(Debug, PartialEq)]
pub enum ParserError {
    IllegalOperator(usize, char),
    UnknownIdentifier(usize, String),
    MissingBracket(usize),
    MissingShift(usize),
    KeywordError(usize, String),
    RadixError(usize, String),
}

#[derive(Debug, PartialEq)]
pub enum EvaluatorError {
}
