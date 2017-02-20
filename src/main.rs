extern crate bspl;
extern crate tempfile;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use tempfile::NamedTempFile;
use bspl::constants::VERSION;
use bspl::lexer::Lexer;
use bspl::parser::Parser;
use bspl::evaluator::Evaluator;
use bspl::error::{ParserError, EvaluatorError};

fn prelude() {
    println!("bspl {}", VERSION);
    println!("Bit-Shift-Print Loop");
    println!("Type 'help', or 'license' for more information.");
}

fn error_message(width: usize, msg: &'static str) {
    println!("{caret:>width$}\n.. {}", msg, caret = "^", width = width);
}

fn display_results(x: &Vec<String>) {
    for r in x {
        println!(".. {}", r);
    }

    let y = x.last().unwrap();
    println!(".. Dec: {}", y);
    let h: i32 = y.parse().unwrap();
    println!(".. Hex: 0x{:x}", h);
    println!(".. Bin: 0b{:b}", h);

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
                            _ => continue,
                        }
                    }
                    Err(ParserError::IllegalOperator(position)) => {
                        error_message(position + prompt.len() + 1, "Illegal Operator");
                    }
                    Err(ParserError::MissingOpeningBracket(position)) => {
                        error_message(position + prompt.len() + 1, "Missing Opening Bracket");
                    }
                    Err(ParserError::MissingClosingBracket(position)) => {
                        error_message(position + prompt.len() + 1, "Missing Closing Bracket");
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
