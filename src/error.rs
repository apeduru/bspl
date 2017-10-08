#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnknownOperator(usize),
    RadixError(usize),
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    MissingOpeningBracket(usize),
    MissingClosingBracket(usize),
    KeywordError(usize),
}

#[derive(Debug, PartialEq)]
pub enum EvaluatorError {
    MissingArgument(usize),
    TooManyArguments,
    OverflowShift(usize),
    UnknownKeyword(usize),
    Exit,
}
