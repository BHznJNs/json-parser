use std::{iter::Peekable, vec::IntoIter};

use crate::{
    public::position::Position,
    tokenizer::{Symbol, Token, TokenType},
    Array, MatchedSymbol, ParseResult, UnmatchedSymbolError,
};

use super::{match_matched_symbol, ResolvedPair};

pub fn resolve_array(iter: &mut Peekable<IntoIter<Token>>, mut position: Position) -> ParseResult<ResolvedPair<Array>> {
    let mut result_arr = Array::new();

    let temp_pair = super::resolve_value(iter, position)?;
    position = temp_pair.0;
    result_arr.push(temp_pair.1);

    while let Some(&Token {
        r#type: TokenType::Symbol(Symbol::Comma),
        ..
    }) = iter.peek()
    {
        iter.next().unwrap(); // skip comma

        let temp_pair = super::resolve_value(iter, position)?;
        position = temp_pair.0;
        result_arr.push(temp_pair.1);
    }

    match_matched_symbol(iter.next(), MatchedSymbol::Bracket, position)?;
    return Ok((position, result_arr));
}
