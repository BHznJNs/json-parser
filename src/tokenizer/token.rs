use crate::public::position::Position;
use crate::Number;

use super::symbol::Symbol;
use super::constant::Constant;

#[derive(Clone, Debug)]
pub enum TokenType {
    Symbol(Symbol),
    Number(Number),
    String(String),
    Constant(Constant),
}

#[derive(Clone, Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub position: Position,
}
