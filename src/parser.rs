use crate::tokenizer::{Token, Tokenizer};
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Number(f64),
    Binary {
        right: Box<Node>,
        left: Box<Node>,
        operator: Token,
    },
    Unary {
        right: Box<Node>,
        operator: Token,
    },
    FunctionCall {
        name: String,
        args: Vec<Node>,
    },
    Variable(String),
    Assignment {
        name: String,
        value: Box<Node>,
    },
}

pub struct Parser {
    tokenizer: Tokenizer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(mut tokenizer: Tokenizer) -> Parser {
        let current_token = tokenizer.next();
        Parser {
            tokenizer,
            current_token,
        }
    }

    fn current_token(&self) -> Option<Token> {
        self.current_token.clone()
    }

    pub fn parse_expression(&mut self) -> Node {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Node {
        let mut expr = self.parse_addition();
        if self.current_token() == Some(Token::Equal) {
            self.advance();
            let value = self.parse_addition();
            if let Node::Variable(name) = expr {
                expr = Node::Assignment {
                    name,
                    value: Box::new(value),
                }
            } else {
                panic!("Invalid assignment");
            }
        }
        expr
    }

    fn parse_addition(&mut self) -> Node {
        let mut expr = self.parse_multiplication();
        while self.current_token() == Some(Token::Plus) || self.current_token() == Some(Token::Minus) {
            let op = self.current_token().unwrap();
            self.advance();
            let right = self.parse_multiplication();
            expr = Node::Binary {
                right: Box::new(expr),
                left: Box::new(right),
                operator: op,
            };
        }
        expr
    }

    fn parse_multiplication(&mut self) -> Node {
        let mut expr = self.parse_unary();
        while self.current_token() == Some(Token::Asterisk) || self.current_token() == Some(Token::Slash) {
            let op = self.current_token().unwrap();
            self.advance();
            let right = self.parse_unary();
            expr = Node::Binary {
                right: Box::new(expr),
                left: Box::new(right),
                operator: op,
            };
        }
        expr
    }

    fn parse_unary(&mut self) -> Node {
        if self.current_token() == Some(Token::Minus) {
            let op = self.current_token().unwrap();
            self.advance();
            let right = self.parse_unary();
            Node::Unary {
                right: Box::new(right),
                operator: op,
            }
        } else {
            self.parse_exponentiation()
        }
    }

    fn parse_exponentiation(&mut self) -> Node {
        let mut expr = self.parse_primary();
        if self.current_token() == Some(Token::Caret) {
            let op = self.current_token().unwrap();
            self.advance();
            let right = self.parse_exponentiation();
            expr = Node::Binary {
                right: Box::new(right),
                left: Box::new(expr),
                operator: op,
            };
        }
        expr
    }

    fn parse_primary(&mut self) -> Node {
        if let Some(Token::Number(n)) = self.current_token() {
            self.advance();
            Node::Number(n)
        } else if let Some(Token::Identifier(name)) = self.current_token() {
            self.advance();
            if self.current_token() == Some(Token::OpenParen) {
                self.advance();
                let mut args = Vec::new();
                args.push(self.parse_expression());
                while self.current_token() != Some(Token::CloseParen) {
                    if self.current_token() != Some(Token::Comma) {
                        panic!("Expected comma");
                    }
                    self.advance();
                    args.push(self.parse_expression());
                }
                self.advance();
                Node::FunctionCall { name, args }
            } else {
                Node::Variable(name)
            }
        } else if self.current_token() == Some(Token::OpenParen) {
            self.advance();
            let expr = self.parse_expression();
            if self.current_token() != Some(Token::CloseParen) {
                panic!("Expected closing parenthesis");
            }
            self.advance();
            expr
        } else if self.current_token() == Some(Token::Pipe) {
            self.advance();
            let expr = self.parse_expression();
            if self.current_token() != Some(Token::Pipe) {
                panic!("Expected closing pipe");
            }
            self.advance();
            Node::FunctionCall {
                name: "abs".to_string(),
                args: vec![expr],
            }
        } else {
            panic!("Expected number or opening parenthesis");
        }
    }

    fn advance(&mut self) {
        self.current_token = self.tokenizer.next();
    }

}