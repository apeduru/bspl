#[derive(Debug, PartialEq)]
pub enum ParserError {
    IllegalOperator(usize),
    UnknownIdentifier(usize),
    MissingBracket(usize),
    RadixError(usize),
}

#[derive(Debug, PartialEq)]
pub enum EvaluatorError {
}
