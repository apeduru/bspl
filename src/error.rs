#[derive(Debug, PartialEq)]
pub enum ParserError {
    RadixError(usize),
    UnknownOperator(usize),
    InvalidSyntax(usize),
    MissingOpeningBracket(usize),
    MissingClosingBracket(usize),
}

#[derive(Debug, PartialEq)]
pub enum EvaluatorError {
    MissingArgument(usize),
    TooManyArguments,
    NegativeShift(usize),
    KeywordError(usize),
    UnknownKeyword(usize),
    Exit,
}
