use crate::error::{ParseError, Result};


pub struct PrattParser<'a> {
    tokens: &'a [Token],  // The list of tokens to parse
    pos: usize,           // Current position in the token stream
}

impl<'a> PrattParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_expression(&mut self, rbp: u8) -> Result<Expr> {
        // TODO: Implement the Pratt parsing algorithm here.
        Ok(Expr::Literal(42))  // Dummy return value for now
    }
}


#[derive(Debug)]
pub enum Token {

}

#[derive(Debug)]
pub enum Expr {
    Literal(i32),

}
