use std::collections::HashMap;
use std::hash::Hash;
use crate::parser::Node;
use crate::parser::Node::Number;
use crate::tokenizer::Token;

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(&self, node: Node) -> Node {
        match node {
            Node::Number(_n) => node,
            Node::Binary { right, left, operator} => self.eval_binary(*left, *right, operator),
            Node::Unary { right, operator } => self.eval_unary(*right, operator),
            Node::Assignment { name, value } => self.eval_assignment(name, *value),
            Node::FunctionCall { name, args } => self.eval_function_call(name, args),
            _ => {
                panic!("Invalid node");
            }
        }
    }

    fn eval_binary(&self, left: Node, right: Node, operator: Token) -> Node {
        let left = self.evaluate(left);
        let right = self.evaluate(right);
        if let (Node::Number(left), Node::Number(right)) = (left, right) {
            match operator {
                Token::Plus => Node::Number(left + right),
                Token::Minus => Node::Number(left - right),
                Token::Asterisk => Node::Number(left * right),
                Token::Slash => Node::Number(left / right),
                Token::Caret => Node::Number(left.powf(right)),
                _ => panic!("Invalid binary operator"),
            }
        } else {
            panic!("Invalid operands for binary operator");
        }
    }

    fn eval_unary(&self, right: Node, operator: Token) -> Node {
        let right = self.evaluate(right);
        if let Node::Number(right) = right {
            match operator {
                Token::Minus => Node::Number(-right),
                _ => panic!("Invalid unary operator"),
            }
        } else {
            panic!("Invalid operand for unary operator");
        }
    }

    fn eval_function_call(&self, name: String, args: Vec<Node>) -> (Node) {
        let mut parsed_args = Vec::new();
        for arg in args.clone() {
            if let Node::Number(n) = self.evaluate(arg) {
                parsed_args.push(n);
            } else {
                panic!("Invalid argument for function");
            }
        }
        let args = parsed_args;
        match name.as_str() {
            "root" => {
                Number(args[0].powf(1.0 / args[1]))
            }
            "sin" => {
                Number(args[0].sin())
            }
            "cos" => {
                Number(args[0].cos())
            }
            "tan" => {
                Number(args[0].tan())
            }
            "asin" => {
                Number(args[0].asin())
            }
            "acos" => {
                Number(args[0].acos())
            }
            "atan" => {
                Number(args[0].atan())
            }
            "sqrt" => {
                Number(args[0].sqrt())
            }
            "ln" => {
                Number(args[0].ln())
            }
            "log" => {
                if args.len() == 1 {
                    Number(args[0].log10())
                } else {
                    Number(args[0].log(args[1]))
                }
            }
            "abs" => {
                Number(args[0].abs())
            }
            _ => {
                panic!("Invalid function name");
            }
        }
    }

    fn eval_assignment(&self, name: String, value: Node) -> Node {
        let value = self.evaluate(value);
        if let Node::Number(value) = value {
            Node::Number(value)
        } else {
            panic!("Invalid assignment");
        }
    }

}