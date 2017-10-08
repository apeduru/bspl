#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnknownOperator(usize),
    RadixError(usize),
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    MissingOpeningBracket(usize),
    MissingClosingBracket(usize),
}

#[derive(Debug, PartialEq)]
pub enum EvaluatorError {
    MissingArgument(usize),
    TooManyArguments,
    OverflowShift(usize),
    KeywordError(usize),
    UnknownKeyword(usize),
    Exit,
}
