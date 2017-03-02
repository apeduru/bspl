#[derive(Debug, PartialEq)]
pub enum ParserError {
    RadixError(usize),
    UnknownOperator(usize),
    MissingOpeningBracket(usize),
    MissingClosingBracket(usize),
}

#[derive(Debug, PartialEq)]
pub enum EvaluatorError {
    MissingArgument(usize),
}
