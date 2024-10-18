use super::position::Position;

pub trait ParseError: std::fmt::Debug {
    fn error_msg(&self) -> String;
}

// --- --- --- --- --- ---

#[derive(Debug)]
pub struct UnexpectedCharError {
    ch: char,
    position: Position,
}
impl UnexpectedCharError {
    pub fn new(ch: char, position: Position) -> Box<Self> {
        Box::new(Self { ch, position })
    }
}
impl ParseError for UnexpectedCharError {
    fn error_msg(&self) -> String {
        format!("Unexpected character at {}: {}.", self.position, self.ch)
    }
}

// --- --- --- --- --- ---

#[derive(Debug)]
pub struct UnexpectedEscapeCharError {
    ch: char,
    position: Position,
}
impl UnexpectedEscapeCharError {
    pub fn new(ch: char, position: Position) -> Box<Self> {
        Box::new(Self { ch, position })
    }
}
impl ParseError for UnexpectedEscapeCharError {
    fn error_msg(&self) -> String {
        format!("Unexpected escape character at {}: '\\{}'.", self.position, self.ch)
    }
}

// --- --- --- --- --- ---

#[derive(Debug)]
pub struct UnmatchedQuoteError {
    lexeme: String,
    position: Position,
}
impl UnmatchedQuoteError  {
    pub fn new(lexeme: String, position: Position) -> Box<Self> {
        Box::new(Self { lexeme, position })
    }
}
impl ParseError for UnmatchedQuoteError  {
    fn error_msg(&self) -> String {
        format!("Unmatched quote at position {}: {}.", self.position, self.lexeme)
    }
}

// --- --- --- --- --- ---

#[derive(Debug)]
pub struct UnexpectedIdentifierError {
    identifier: String,
}
impl UnexpectedIdentifierError {
    pub fn new(identifier: String) -> Box<Self> {
        Box::new(Self { identifier })
    }
}
impl ParseError for UnexpectedIdentifierError {
    fn error_msg(&self) -> String {
        format!("Unexpected identifier: {}.", self.identifier)
    }
}

// --- --- --- --- --- ---

#[derive(Debug)]
pub struct UnexpectedTokenError {
    lexeme: String,
    position: Position,
}
impl UnexpectedTokenError {
    pub fn new(lexeme: String, position: Position) -> Box<Self> {
        Box::new(Self { lexeme, position })
    }
}
impl ParseError for UnexpectedTokenError {
    fn error_msg(&self) -> String {
        format!("Unexpected token at position {}: {}.", self.position, self.lexeme)
    }
}

// --- --- --- --- --- ---

pub type GenericParseError = Box<dyn ParseError>;
pub type ParseResult<T> = Result<T, GenericParseError>;
