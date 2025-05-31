#[derive(Debug, Clone)]
pub enum Token {
    Select,
    From,
    Where,
    OrderBy,
    Identifier(String),
    Number(i64),
    Star,
    Comma,
    Semicolon,
    Operator(String),
    Asc,
    Desc,
    EOF,

}

pub struct Tokenizer<'a> {
    input: &'a str,

}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        vec![Token::EOF]
    }
}
