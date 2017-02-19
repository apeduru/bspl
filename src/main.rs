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
// use bspl::error::ParserError;

fn prelude() {
    println!("bspl {}", VERSION);
    println!("Bit-Shift-Print Loop");
    println!("Type 'help', or 'license' for more information.");
}

fn repl() {
    let mut repl = Editor::<()>::new();
    let tmp_file = NamedTempFile::new().unwrap();
    let _ = repl.load_history(tmp_file.path());
    let mut lexer = Lexer::new();
    let mut parser = Parser::default();
    // let mut evaluator = Evaluator::new();

    loop {
        match repl.readline("=> ") {
            Ok(line) => {
                repl.add_history_entry(&line);
                let mut tokens = lexer.analyze(&line);
                for t in tokens.iter() {
                    println!("{:?}", t);
                }
                let mut parsed_tokens = parser.parse(tokens);
                for p in parsed_tokens.iter() {
                    println!("{:?}", p);
                }
                // let result: String = evaluator.evaluate(parsed_tokens);
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
