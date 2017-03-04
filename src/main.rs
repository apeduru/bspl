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
use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;
use error::{ParserError, EvaluatorError};

fn prelude() {
    println!("bspl {}", VERSION);
    println!("Bit-Shift-Print Loop");
}

fn error_message(width: usize, msg: &'static str) {
    println!("{caret:>width$}\n.. {}", msg, caret = "^", width = width);
}

fn display_results(results: &Vec<String>) {
    for result in results {
        println!(".. {}", result);
    }
    let final_result: i32 = results.last().unwrap().parse().unwrap();
    println!(".. Dec: {}", final_result);
    println!(".. Hex: 0x{:x}", final_result);
    println!(".. Bin: 0b{:b}", final_result);
}

fn repl() {
    let mut repl = Editor::<()>::new();
    let prompt = "=> ";
    let tmp_file = NamedTempFile::new().unwrap();
    let _ = repl.load_history(tmp_file.path());

    let mut lexer = Lexer::new();
    let mut parser = Parser::default();
    let mut evaluator = Evaluator::default();

    loop {
        match repl.readline(prompt) {
            Ok(line) => {
                repl.add_history_entry(&line);
                match parser.parse(lexer.analyze(&line)) {
                    Ok(parsed_tokens) => {
                        match evaluator.evaluate(parsed_tokens) {
                            Ok(result) => {
                                display_results(&result);
                            }
                            Err(EvaluatorError::MissingArgument(position)) => {
                                error_message(position + prompt.len() + 1, "Missing Argument");
                            }
                            Err(EvaluatorError::TooManyArguments) => {
                                error_message(prompt.len() + 1, "Too Many Arguments");
                            }
                        }
                    }
                    Err(ParserError::RadixError(position)) => {
                        error_message(position + prompt.len() + 1, "Radix Error");
                    }
                    Err(ParserError::UnknownOperator(position)) => {
                        error_message(position + prompt.len() + 1, "Unknown Operator");
                    }
                    Err(ParserError::MissingOpeningBracket(position)) => {
                        error_message(position + prompt.len() + 1, "Missing Opening Bracket");
                    }
                    Err(ParserError::MissingClosingBracket(position)) => {
                        error_message(position + prompt.len() + 1, "Missing Closing Bracket");
                    }
                    Err(ParserError::InvalidSyntax(position)) => {
                        error_message(position + prompt.len() + 1, "Invalid Syntax");
                    }
                }
            }
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => {
                println!("Press Ctrl-D to leave bspl");
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
