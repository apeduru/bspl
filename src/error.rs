#[derive(Debug, PartialEq)]
pub enum ParserError {
    RadixError(usize),
    UnknownOperator(usize),
    MissingOpeningBracket(usize),
    MissingClosingBracket(usize),
}

#[derive(Debug, PartialEq)]
pub enum EvaluatorError {
    // KeywordError(usize, String),
    // UnknownIdentifier(usize, String),
    MissingArgument(usize),
}
