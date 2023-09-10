use std::io::{self, stdin, Read, Write};

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
        let tokens = gettok(input.as_str());
        println!("{:?}", tokens);
        continue;
    }
}

#[derive(Debug)]
enum Token {
    Eof,
    // commands
    Def,
    Extern,
    // primary
    Ident(String),
    Number(f64),
}

// lexer
fn gettok(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut input_itr = input.chars();
    let mut next_input = || -> Option<char> { input_itr.next() };
    'main: loop {
        let _c = next_input();
        if _c.is_none() {
            break;
        }
        let c = _c.unwrap();
        if c.is_whitespace() {
            continue;
        }
        // ident: [a-zA-Z][a-zA-Z0-9]*
        if c.is_alphabetic() {
            let mut ident_chars: Vec<char> = vec![c];
            loop {
                let _next_c = next_input();
                if _next_c.is_none() {
                    break 'main;
                }
                let next_c = _next_c.unwrap();
                if next_c.is_alphanumeric() {
                    ident_chars.push(next_c);
                } else {
                    let ident_str: String = ident_chars.into_iter().collect();
                    match &*ident_str {
                        "def" => tokens.push(Token::Def),
                        "extern" => tokens.push(Token::Extern),
                        _ => tokens.push(Token::Ident(ident_str)),
                    }
                    break;
                }
            }
        }
    }
    tokens
}
