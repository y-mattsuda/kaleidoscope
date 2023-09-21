mod lexer;

use crate::lexer::Lexer;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        print!("> ");
        stdout.flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).expect("failed to read line");
        if input.as_str() == ".quit\n" {
            break;
        }
        let tokens = Lexer::new(&input).tokenize().unwrap();
        println!("{:?}", tokens);
        continue;
    }
}
