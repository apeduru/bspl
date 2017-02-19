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
use bspl::error::ParserError;

fn prelude() {
    println!("bspl {}", VERSION);
    println!("Bit-Shift-Print Loop");
    println!("Type 'help', or 'license' for more information.");
}

fn repl() {
    let mut repl = Editor::<()>::new();
    let prompt = "=> ";
    let tmp_file = NamedTempFile::new().unwrap();
    let _ = repl.load_history(tmp_file.path());

    let mut lexer = Lexer::new();
    let mut parser = Parser::default();
    // let mut evaluator = Evaluator::new();

    loop {
        match repl.readline(prompt) {
            Ok(line) => {
                repl.add_history_entry(&line);
                match parser.parse(lexer.analyze(&line)) {
                    Ok(parsed_tokens) => {
                        println!("{:?}", parsed_tokens);
                        // let result: String = evaluator.evaluate(parsed_tokens);
                    }
                    Err(ParserError::IllegalOperator(position)) => {
                        println!("{caret:>width$}\n.. Illegal Operator",
                                 caret = "^",
                                 width = position + prompt.len() + 1);
                    }
                    Err(ParserError::MissingBracket(position)) => {
                        println!("{caret:>width$}\n.. Missing Bracket",
                                 caret = "^",
                                 width = position + prompt.len() + 1);
                    }
                    _ => break,

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
