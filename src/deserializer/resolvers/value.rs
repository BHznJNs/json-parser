use std::{iter::Peekable, vec::IntoIter};

use crate::{
    public::position::Position, tokenizer::{Symbol, Token, TokenType}, IntoValue, ParseResult, UnexpectedEOFError, UnexpectedTokenError, Value
};

use super::ResolvedPair;

pub fn resolve_value(iter: &mut Peekable<IntoIter<Token>>, mut position: Position) -> ParseResult<ResolvedPair<Value>> {
    let Some(token) = iter.next() else {
        return Err(UnexpectedEOFError::new(position))
    };
    position = token.position;

    let result_val = match &token.r#type {
        TokenType::Symbol(Symbol::LeftBrace) => {
            let temp_pair = super::resolve_object(iter, position)?;
            position = temp_pair.0;
            Value::Object(temp_pair.1)
        }
        TokenType::Symbol(Symbol::LeftBracket) => {
            let temp_pair = super::resolve_array(iter, position)?;
            position = temp_pair.0;
            Value::Array(temp_pair.1)
        }
        TokenType::Number(number_val) => number_val.into_val(),
        TokenType::String(string_val) => string_val.clone().into_val(),
        TokenType::Constant(constant_val) => constant_val.into_val(),
        _ => return Err(UnexpectedTokenError::new(token.clone())),
    };
    return Ok((position, result_val));
}
