#[derive(Debug, PartialEq)]
pub enum ParserError {
    IllegalOperator(usize),
    MissingOpeningBracket(usize),
    MissingClosingBracket(usize),
}

#[derive(Debug, PartialEq)]
pub enum EvaluatorError {
    // RadixError(usize),
    // KeywordError(usize, String),
    // UnknownIdentifier(usize, String),
    MissingArgument(usize),
}
