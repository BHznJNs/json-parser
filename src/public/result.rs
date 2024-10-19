use crate::tokenizer::Token;

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
pub enum MatchedSymbol {
    Quote,
    Bracket,
    Brace,
}

#[derive(Debug)]
pub struct UnmatchedSymbolError {
    r#type: MatchedSymbol,
    position: Position,
}
impl UnmatchedSymbolError  {
    pub fn new(r#type: MatchedSymbol, position: Position) -> Box<Self> {
        Box::new(Self { r#type, position })
    }
}
impl ParseError for UnmatchedSymbolError  {
    fn error_msg(&self) -> String {
        format!("Unmatched {} at position {}.", match self.r#type {
            MatchedSymbol::Quote => "quote",
            MatchedSymbol::Bracket => "bracket",
            MatchedSymbol::Brace => "brace",
        }, self.position)
    }
}

// --- --- --- --- --- ---

#[derive(Debug)]
pub struct UnexpectedIdentifierError {
    identifier: String,
    position: Position,
}
impl UnexpectedIdentifierError {
    pub fn new(identifier: String, position: Position) -> Box<Self> {
        Box::new(Self { identifier, position })
    }
}
impl ParseError for UnexpectedIdentifierError {
    fn error_msg(&self) -> String {
        format!("Unexpected identifier at {}: {}.", self.position, self.identifier)
    }
}

// --- --- --- --- --- ---

#[derive(Debug)]
pub struct UnexpectedTokenError {
    lexeme: String,
    position: Position,
}
impl UnexpectedTokenError {
    pub fn new(token: Token) -> Box<Self> {
        Box::new(Self { lexeme: token.lexeme, position: token.position })
    }
}
impl ParseError for UnexpectedTokenError {
    fn error_msg(&self) -> String {
        format!("Unexpected token at position {}: {}.", self.position, self.lexeme)
    }
}

// --- --- --- --- --- ---

#[derive(Debug)]
pub struct UnexpectedEOFError {
    position: Position,
}
impl UnexpectedEOFError {
    pub fn new(position: Position) -> Box<Self> {
        Box::new(Self { position })
    }
}
impl ParseError for UnexpectedEOFError {
    fn error_msg(&self) -> String {
        format!("Unexpected EOF at position {}.", self.position)
    }
}

// --- --- --- --- --- ---

pub type GenericParseError = Box<dyn ParseError>;
pub type ParseResult<T> = Result<T, GenericParseError>;
