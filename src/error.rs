#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnknownOperator(usize),
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    RadixError(usize),
    InvalidSyntax(usize),
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
