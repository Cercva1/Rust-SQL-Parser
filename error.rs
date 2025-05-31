#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEOF,
    InvalidSyntax(String),
    Other(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;
