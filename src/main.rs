extern crate fnv;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate rustyline;

use rustyline::Editor;
use rustyline::error::ReadlineError;

use crate::types::format_error;

#[macro_use]
mod types;
mod printer;
mod reader;

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history(".mal-history").is_err() {
        eprintln!("No previous history.");
    }

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                rl.save_history(".mal-history").unwrap();
                if line.len() > 0 {
                    match reader::read_str(line) {
                        Ok(mv) => {
                            println!("{}", mv.pr_str(true));
                        }
                        Err(e) => println!("Error: {}", format_error(e)),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
