#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenParen,
    CloseParen,
    Asterisk,
    Caret,
    Plus,
    Minus,
    Slash,
    Comma,
    Pipe,
    Equal,
    Number(f64),
    Identifier(String),
}

pub struct Tokenizer {
    start: usize,
    input: String,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        Tokenizer {
            start: 0,
            input,
            position: 0,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.start = self.position;
        match self.input.chars().nth(self.position) {
            Some(c) => {
                match c {
                    '(' => self.make_token(Token::OpenParen),
                    ')' => self.make_token(Token::CloseParen),
                    '*' => self.make_token(Token::Asterisk),
                    '^' => self.make_token(Token::Caret),
                    '+' => self.make_token(Token::Plus),
                    '-' => self.make_token(Token::Minus),
                    '/' => self.make_token(Token::Slash),
                    ',' => self.make_token(Token::Comma),
                    '|' => self.make_token(Token::Pipe),
                    '=' => self.make_token(Token::Equal),
                    '0'..='9' => self.make_number(),
                    'a'..='z' | 'A'..='Z' => self.make_identifier(),
                    _ => panic!("Unexpected character: {}", c),
                }
            }
            None => None,
        }
    }

    fn make_number(&mut self) -> Option<Token> {
        let mut dot = false;
        while let Some(c) = self.input.chars().nth(self.position) {
            match c {
                '0'..='9' => self.position += 1,
                '.' => {
                    if dot {
                        panic!("Unexpected character: {}", c);
                    }
                    dot = true;
                    self.position += 1;
                }
                _ => break,
            }
        }
        let number = self.input[self.start..self.position].parse::<f64>().unwrap();
        Some(Token::Number(number))
    }

    fn make_token(&mut self, token: Token) -> Option<Token> {
        self.position += 1;
        Some(token)
    }

    fn make_identifier(&mut self) -> Option<Token> {
        while let Some(c) = self.input.chars().nth(self.position) {
            match c {
                'a'..='z' | 'A'..='Z' => self.position += 1,
                _ => break,
            }
        }
        let identifier = self.input[self.start..self.position].to_string();
        Some(Token::Identifier(identifier))
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.input.chars().nth(self.position) {
            match c {
                ' ' | '\r' | '\t' => self.position += 1,
                _ => break,
            }
        }
    }
}