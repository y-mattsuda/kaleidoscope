#[derive(Debug, PartialEq)]
pub enum Token {
    WhiteSpace,
    Comment(String),
    // commands
    Def,
    Extern,
    // primary
    Ident(String),
    Number(f64),
}

pub struct Lexer<'a> {
    // 読み込み中の先頭文字列
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}
#[derive(Debug)]
pub struct LexerError {
    pub msg: String,
}

impl LexerError {
    fn new(msg: &str) -> Self {
        LexerError { msg: msg.into() }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::<Token>::new();
        while let Some(token) = self.next_token()? {
            match token {
                Token::WhiteSpace => {}
                // Token::Comment(_) => {}
                _ => tokens.push(token),
            }
        }
        Ok(tokens)
    }

    // 一文字分だけ読み進めてTokenを返す
    fn next_return_token(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    // 先頭の文字からマッチしたトークンを返す
    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        match self.chars.peek() {
            Some(c) => match c {
                // ' 'もしくは'\n'はWhiteSpace
                c if c.is_whitespace() || *c == '\n' => {
                    Ok(self.next_return_token(Token::WhiteSpace))
                }
                // number: ([0-9]|'+'|'-'|'.')[0-9]*
                c if c.is_numeric() || matches!(c, '+' | '-' | '.') => self.parse_number_token(),
                // ident: [a-zA-Z][a-zA-Z0-9]*
                c if c.is_alphabetic() => self.parse_ident_token(),
                '#' => self.parse_comment_token(),
                _ => Err(LexerError::new(&format!("error: {}", c))),
            },
            None => Ok(None),
        }
    }

    fn parse_number_token(&mut self) -> Result<Option<Token>, LexerError> {
        let mut number_str = String::new();
        while let Some(&c) = self.chars.peek() {
            // 1e10, 1E10, 1.000など
            if c.is_numeric() | matches!(c, '+' | '-' | 'e' | 'E' | '.') {
                self.chars.next();
                number_str.push(c);
            } else {
                break;
            }
        }
        match number_str.parse::<f64>() {
            Ok(number) => Ok(Some(Token::Number(number))),
            Err(e) => Err(LexerError::new(&format!("error: {}, input: '{}'", e, number_str))),
        }
    }

    fn parse_ident_token(&mut self) -> Result<Option<Token>, LexerError> {
        let mut ident_str = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() {
                self.chars.next();
                ident_str.push(c);
            } else {
                break;
            }
        }
        match &*ident_str {
            "def" => Ok(Some(Token::Def)),
            "extern" => Ok(Some(Token::Extern)),
            _ => Ok(Some(Token::Ident(ident_str))),
        }
    }

    fn parse_comment_token(&mut self) -> Result<Option<Token>, LexerError> {
        let mut comment_str = String::new();
        // 先頭の#は読み飛ばす
        self.chars.next();
        while let Some(&c) = self.chars.peek()  {
            if c != '\n' && c != '\r' {
                self.chars.next();
                comment_str.push(c);
            } else {
                break;
            }
        }
        Ok(Some(Token::Comment(comment_str)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tokenize {
        ($input:expr) => {
            Lexer::new($input).tokenize().unwrap()
        };
    }

    #[test]
    fn test_number() {
        // integer
        let tokens = tokenize!("1234567890");
        assert_eq!(tokens[0], Token::Number(1234567890f64));
        let tokens = tokenize!("+123");
        assert_eq!(tokens[0], Token::Number(123f64));
        // float
        let tokens = tokenize!("-0.001");
        assert_eq!(tokens[0], Token::Number(-0.001));
        let tokens = tokenize!(".001");
        assert_eq!(tokens[0], Token::Number(0.001));
        // exponent
        let tokens = tokenize!("1e-10");
        assert_eq!(tokens[0], Token::Number(0.0000000001));
        let tokens = tokenize!("+2E10");
        assert_eq!(tokens[0], Token::Number(20000000000f64))
    }

    #[test]
    fn test_ident() {
        // keyword
        let tokens = tokenize!("def");
        assert_eq!(tokens[0], Token::Def);
        let tokens = tokenize!("extern");
        assert_eq!(tokens[0], Token::Extern);
        let tokens = tokenize!("rust");
        assert_eq!(tokens[0], Token::Ident("rust".to_owned()))
    }

    #[test]
    fn test_comment() {
        let tokens = tokenize!("#this is comment");
        assert_eq!(tokens[0], Token::Comment("this is comment".to_owned()));
        let tokens = tokenize!("#a\na");
        assert_eq!(tokens, vec![Token::Comment("a".to_owned()), Token::Ident("a".to_owned())]);
    }
}
