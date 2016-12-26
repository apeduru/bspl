extern crate rustyline;
extern crate tempfile;
extern crate bspl;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use tempfile::NamedTempFile;
use bspl::keywords::{VERSION, LICENSE, HELP};

fn prelude() {
    println!("bspl {}", VERSION);
    println!("Bit-Shift-Print Loop");
    println!("Type 'help', or 'license' for more information.");
}

fn process_line(line: &str) {
    if line == "license" {
        println!("{}", LICENSE);
    } else if line == "help" {
        println!("{}", HELP);
    } else {
        println!("{}", line);
    }
}

fn repl() {
    let mut repl = Editor::<()>::new();
    let tmp_file = NamedTempFile::new().unwrap();
    let _ = repl.load_history(tmp_file.path());

    loop {
        let readline = repl.readline("=> ");
        match readline {
            Ok(line) => {
                repl.add_history_entry(&line);
                process_line(&line);
            }
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => {
                println!("KeyboardInterrupt");
                continue;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        repl.save_history(tmp_file.path()).unwrap();
    }
    tmp_file.close();
}

pub fn main() {
    prelude();
    repl();
}
