mod tokenizer;
mod parser;
mod evaluator;
mod functions;

use tokenizer::Tokenizer;
use parser::Parser;
use evaluator::Evaluator;
use parser::Node;

fn main() {
    let tokenizer = Tokenizer::new(expr.to_string());
    let mut parser = Parser::new(tokenizer);
    let evaluator = Evaluator;

    let ast = parser.parse_expression();
    let result = evaluator.evaluate(ast);
    if let Node::Number(n) = result {
        println!("{}", n);
    } else {
        panic!("Invalid result");
    }
}