use std::{iter::Peekable, vec::IntoIter};

use crate::{
    public::position::Position,
    tokenizer::{Symbol, Token, TokenType},
    MatchedSymbol, Object, ParseResult, Value
};

use super::{match_key, match_matched_symbol, match_symbol, ResolvedPair};

type KVPair = (String, Value);

fn resolve_kv_pair(iter: &mut Peekable<IntoIter<Token>>, mut position: Position) -> ParseResult<ResolvedPair<KVPair>> {
    let key = match_key(iter.next(), position)?;
    position = match_symbol(iter.next(), Symbol::Colon, position)?;

    let temp_pair = super::resolve_value(iter, position)?;
    position = temp_pair.0;
    return Ok((position, (key, temp_pair.1)));
}

pub fn resolve_object(iter: &mut Peekable<IntoIter<Token>>, mut position: Position) -> ParseResult<ResolvedPair<Object>> {
    let mut result_obj = Object::new();

    let (temp_position, temp_kv_pair) = resolve_kv_pair(iter, position)?;
    position = temp_position;
    result_obj.insert(temp_kv_pair.0, Box::new(temp_kv_pair.1));

    while let Some(&Token {
        r#type: TokenType::Symbol(Symbol::Comma),
        ..
    }) = iter.peek()
    {
        iter.next().unwrap(); // skip comma

        let (temp_position, temp_kv_pair) = resolve_kv_pair(iter, position)?;
        position = temp_position;
        result_obj.insert(temp_kv_pair.0, Box::new(temp_kv_pair.1));
    }
    match_matched_symbol(iter.next(), MatchedSymbol::Brace, position)?;
    return Ok((position, result_obj));
}
