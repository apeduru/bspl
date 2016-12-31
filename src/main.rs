extern crate bspl;
extern crate tempfile;
extern crate rustyline;
// extern crate error_chain;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use tempfile::NamedTempFile;
use bspl::constants::VERSION;
use bspl::lexer::Lexer;
// use bspl::parser::Parser;
// use bspl::evaluator;
// use bspl::converter;
// use bspl::error::*;

fn prelude() {
    println!("bspl {}", VERSION);
    println!("Bit-Shift-Print Loop");
    println!("Type 'help', or 'license' for more information.");
}

// fn process_line(result: &str) {
//     let val = result.parse::<i32>().unwrap();
//     println!(".. Dec: {}", val);
//     println!(".. Hex: {:#x}", val);
//     println!(".. Bin: {:#b}", val);
// }

fn repl() {
    let mut repl = Editor::<()>::new();
    let tmp_file = NamedTempFile::new().unwrap();
    let _ = repl.load_history(tmp_file.path());

    let mut lexer = Lexer::new();
    // let mut parser = Parser::new().init();

    loop {
        match repl.readline("=> ") {
            Ok(line) => {
                repl.add_history_entry(&line);
                let tokens = lexer.analyze(&line);
                for t in tokens.iter() {
                    println!("{:?}", t);

                }
                // match parser.parse(tokens) {
                //     Ok(parsed_tokens) => {}
                //     Err(ParseError) => {
                //         println!("{:?}", ParseError);
                //         continue;
                //     }
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
