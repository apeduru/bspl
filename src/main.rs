extern crate rustyline;
// TODO: library for managing tempfiles
// extern crate tempfile;

use rustyline::error::ReadlineError;
use rustyline::Editor;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn help_menu(){

}

fn prelude(){
    println!("bspl {}", VERSION);
    println!("Bit-Shift-Print Loop");
    println!("Type 'help', or 'license' for more information.");
}

fn main(){
    let mut repl = Editor::<()>::new();
    // TODO: create a tempfile to conserve history
    if let Err(_) = repl.load_history("history.txt") {
        // TODO: Probably won't need this guard clause after we create tempfile
        // println!("No previous history.");
    }
    prelude();
    loop {
        let readline = repl.readline("# ");
        match readline {
            Ok(line) => {
                repl.add_history_entry(&line);
                // TODO: function(s) to parse and compute bit shift operations
                // TODO: Help menu
                // TODO: License
                println!("{}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("KeyboardInterrupt");
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    // TODO: Cleanup, delete history tempfile
    repl.save_history("history.txt").unwrap();
}
