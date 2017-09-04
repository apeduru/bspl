extern crate tempfile;
extern crate rustyline;

mod lexer;
mod parser;
mod evaluator;
mod error;
mod constants;
mod function;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use tempfile::NamedTempFile;
use constants::VERSION;
use lexer::lexer;
use parser::Parser;
use evaluator::Evaluator;
use error::{LexerError, ParserError, EvaluatorError};

fn prelude() {
    println!("bspl {}", VERSION);
    println!("Bit-Shift-Print Loop");
    println!("Type 'help', 'license', or 'version' for more information.");
}

fn error_message(width: usize, msg: &str) {
    println!("{caret:>width$}\n.. {}", msg, caret = "^", width = width);
}

fn display_results(results: Vec<String>) {
    let mut result_iter = results.iter();
    while let Some(result) = result_iter.next() {
        println!(".. {}", result);
    }

    if let Some(final_result) = results.last() {
        let parsed_final_result = final_result.parse::<u32>();
        if parsed_final_result.is_ok() {
            let parsed_final_result = parsed_final_result.unwrap();
            println!(".. Dec: {}", parsed_final_result);
            println!(".. Hex: 0x{:x}", parsed_final_result);
            println!(".. Bin: 0b{:b}", parsed_final_result);
        }
    }
}

fn repl() {
    let mut repl = Editor::<()>::new();
    let prompt = "=> ";
    let prompt_len = prompt.len() + 1;
    let tmp_file = NamedTempFile::new().unwrap();
    let _ = repl.load_history(tmp_file.path());

    let mut parser = Parser::default();
    let evaluator = Evaluator::default();

    loop {
        match repl.readline(prompt) {
            Ok(line) => {
                repl.add_history_entry(&line);
                match lexer(&line) {
                    Ok(tokens) => {
                        match parser.parse(tokens) {
                            Ok(parsed_tokens) => {
                                match evaluator.evaluate(parsed_tokens) {
                                    Ok(result) => {
                                        display_results(result);
                                    }
                                    Err(EvaluatorError::MissingArgument(position)) => {
                                        error_message(position + prompt_len, "Missing Argument");
                                    }
                                    Err(EvaluatorError::TooManyArguments) => {
                                        error_message(prompt_len, "Too Many Arguments");
                                    }
                                    Err(EvaluatorError::OverflowShift(position)) => {
                                        error_message(position + prompt_len, "Overflow Shift");
                                    }
                                    Err(EvaluatorError::KeywordError(position)) => {
                                        error_message(position + prompt_len, "Cannot use Keyword");
                                    }
                                    Err(EvaluatorError::UnknownKeyword(position)) => {
                                        error_message(position + prompt_len, "Unknown Keyword");
                                    }
                                    Err(EvaluatorError::Exit) => break,
                                }
                            }
                            Err(ParserError::RadixError(position)) => {
                                error_message(position + prompt_len, "Radix Error");
                            }
                            Err(ParserError::MissingOpeningBracket(position)) => {
                                error_message(position + prompt_len, "Missing Opening Bracket");
                            }
                            Err(ParserError::MissingClosingBracket(position)) => {
                                error_message(position + prompt_len, "Missing Closing Bracket");
                            }
                            Err(ParserError::InvalidSyntax(position)) => {
                                error_message(position + prompt_len, "Invalid Syntax");
                            }
                        }
                    }
                    Err(LexerError::UnknownOperator(position)) => {
                        error_message(position + prompt_len, "Unknown Operator");
                    }
                }
            }
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => {
                println!("Type 'exit' or press Ctrl-D to leave bspl");
                continue;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }

        }
        repl.save_history(tmp_file.path()).unwrap();
    }
}

pub fn main() {
    prelude();
    repl();
}
