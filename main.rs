mod ast;
mod tokenizer;
mod parser;
mod error;
mod pratt;
mod sql;

use parser::Parser;

fn main() {
    let input = "SELECT id, name FROM customers;";
    let mut parser = Parser::new(input);

    match parser.parse_select() {
        Ok(statement) => println!("{:#?}", statement),
        Err(e) => eprintln!("Parse error: {:?}", e),
    }
}
