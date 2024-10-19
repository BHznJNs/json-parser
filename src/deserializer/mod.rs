mod resolvers;

use crate::{
    public::position::Position, tokenizer::Token, ParseResult, UnexpectedTokenError, Value
};

pub fn deserialize(tokens: Vec<Token>) -> ParseResult<Value> {
    let mut token_iter = tokens.into_iter().peekable();
    let (_, result) = resolvers::resolve_value(&mut token_iter, Position::new(0, 0))?;

    if let Some(token) = token_iter.peek() {
        return Err(UnexpectedTokenError::new(token.clone()));
    }
    return Ok(result);
}
