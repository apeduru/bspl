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
            println!("D: {}", parsed_final_result);
            println!("H: {:#x}", parsed_final_result);
            println!("B: {:#b}", parsed_final_result);
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
                                        error_message(position + prompt_len,
                                                      "Missing argument from expression");
                                    }
                                    Err(EvaluatorError::TooManyArguments) => {
                                        error_message(prompt_len,
                                                      "Too many arguments in expression");
                                    }
                                    Err(EvaluatorError::OverflowShift(position)) => {
                                        error_message(position + prompt_len,
                                                      "Shift overflow error");
                                    }
                                    Err(EvaluatorError::KeywordError(position)) => {
                                        error_message(position + prompt_len,
                                                      "Cannot use keyword in expression");
                                    }
                                    Err(EvaluatorError::UnknownKeyword(position)) => {
                                        error_message(position + prompt_len, "Not a valid keyword");
                                    }
                                    Err(EvaluatorError::Exit) => break,
                                }
                            }
                            Err(ParserError::MissingOpeningBracket(position)) => {
                                error_message(position + prompt_len, "Missing an opening bracket");
                            }
                            Err(ParserError::MissingClosingBracket(position)) => {
                                error_message(position + prompt_len, "Missing a closing bracket");
                            }
                            Err(ParserError::InvalidSyntax(position)) => {
                                error_message(position + prompt_len,
                                              "Expression contains invalid syntax");
                            }
                        }
                    }
                    Err(LexerError::RadixError(position)) => {
                        error_message(position + prompt_len,
                                      "Not a recognizable decimal, hexadecimal, or keyword");
                    }
                    Err(LexerError::UnknownOperator(position)) => {
                        error_message(position + prompt_len, "Not a valid operator");
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
