use crate::{tokenizer::{Token, TokenType}, IntoValue, ParseResult, UnexpectedTokenError, Value};

pub fn serialize(tokens: Vec<Token>) -> ParseResult<Value> {
    let mut token_iter = tokens.iter().peekable();
    let mut result = Value::Null;

    while let Some(token) = token_iter.next() {
        if let TokenType::Symbol(symbol) = &token.r#type {
            todo!()
        }

        result = match &token.r#type {
            TokenType::Number(number_val) => number_val.into_val(),
            TokenType::String(string_val) => string_val.clone().into_val(),
            TokenType::Constant(constant_val) => constant_val.into_val(),
            _ => unreachable!(),
        };
        if let Some(&token) = token_iter.peek() {
            return Err(UnexpectedTokenError::new(
                token.lexeme.clone(),
                token.position,
            ));
        }
    }
    return Ok(result);
}
