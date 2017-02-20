#[derive(Debug, PartialEq)]
pub enum ParserError {
    // KeywordError(usize, String),
    IllegalOperator(usize),
    MissingOpeningBracket(usize),
    MissingClosingBracket(usize),
}

#[derive(Debug, PartialEq)]
pub enum EvaluatorError {
    // RadixError(usize),
    UnknownIdentifier(usize, String),
    MissingArgument(usize),
}
