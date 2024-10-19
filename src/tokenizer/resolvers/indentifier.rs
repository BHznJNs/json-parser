use std::{iter::Peekable, str::Chars};

use crate::{public::position::Position, tokenizer::Constant, ParseResult};

use super::ResolvedPair;

pub fn resolve_identifier(iter: &mut Peekable<Chars>, position: &mut Position) -> ParseResult<ResolvedPair<Constant>> {
    let mut result = String::new();
    while iter.peek().is_some_and(|ch| ch.is_alphabetic()) {
        let ch = iter.next().unwrap();
        result.push(ch);
        position.incr_col();
    }
    let lexeme = result.clone();
    let constant_val = Constant::try_from(result)?;
    return Ok((lexeme, constant_val));
}
