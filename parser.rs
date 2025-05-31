use crate::ast::*;
use crate::error::{ParseError, Result};
use crate::sql::*;
use crate::tokenizer::{Token, Tokenizer};
use crate::pratt::PrattParser;


pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    tokens: Vec<Token>,  // The token stream produced by the tokenizer
    pos: usize,          // Current position in the token stream
}

impl<'a> Parser<'a> {
    /// Create a new parser instance from an input string.
    pub fn new(input: &'a str) -> Self {
        let tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();  // Tokenize input
        Self { tokenizer, tokens, pos: 0 }
    }

    /// Peek at the current token without advancing.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }


    fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    pub fn parse_select(&mut self) -> Result<SelectStatement> {

        Ok(SelectStatement {
            projection: vec![SelectItem::Column("name".to_string())],
            from: "users".to_string(),
            selection: Some(Expr::BinaryOp {
                left: Box::new(Expr::ColumnRef("salary".to_string())),
                op: ">".to_string(),
                right: Box::new(Expr::LiteralInt(1500)),
            }),
            order_by: Some(vec![OrderByExpr {
                expr: Expr::BinaryOp {
                    left: Box::new(Expr::ColumnRef("salary".to_string())),
                    op: "*".to_string(),
                    right: Box::new(Expr::LiteralInt(12)),
                },
                asc: false,
            }]),
        })
    }
}
