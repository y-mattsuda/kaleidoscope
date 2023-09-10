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
        if let Some(c) = next_input() {
            if c.is_whitespace() {
                continue;
            }
            // ident: [a-zA-Z][a-zA-Z0-9]*
            if c.is_alphabetic() {
                let mut ident_chars: Vec<char> = vec![c];
                'ident_loop: loop {
                    if let Some(next_c) = next_input() {
                        if next_c.is_alphanumeric() {
                            ident_chars.push(next_c);
                            continue 'ident_loop;
                        } else {
                            let ident_str: String = ident_chars.into_iter().collect();
                            match &*ident_str {
                                "def" => tokens.push(Token::Def),
                                "extern" => tokens.push(Token::Extern),
                                _ => tokens.push(Token::Ident(ident_str)),
                            }
                            break 'ident_loop;
                        }
                    } else {
                        break 'main;
                    }
                }
            }
            if c.is_ascii_digit() || c == '.' {
                let mut num_chars: Vec<char> = vec![c];
                'num_loop: loop {
                    if let Some(next_c) = next_input() {
                        if next_c.is_ascii_digit() || next_c == '.' {
                            num_chars.push(next_c);
                            continue 'num_loop;
                        } else {
                            let num_str: String = num_chars.into_iter().collect();
                            if let Ok(num_val) = num_str.parse::<f64>() {
                                tokens.push(Token::Number(num_val))
                            };
                            break 'num_loop;
                        }
                    } else {
                        break 'main;
                    }
                }
            }
        } else {
            break;
        }
    }
    tokens
}
