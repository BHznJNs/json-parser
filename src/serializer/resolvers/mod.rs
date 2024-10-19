mod value;
mod array;
mod object;

type ResolvedPair<T> = (crate::public::position::Position, T);

pub use value::resolve_value;
pub use array::resolve_array;
pub use object::resolve_object;

use crate::{
    public::position::Position, tokenizer::{Symbol, Token, TokenType}, MatchedSymbol, ParseResult, UnexpectedEOFError, UnexpectedTokenError, UnmatchedSymbolError
};

fn match_matched_symbol(token: Option<Token>, matched_symbol: MatchedSymbol, position: Position) -> ParseResult<()> {
    if token.is_none() {
        return Err(UnexpectedEOFError::new(position));
    }
    
    let target_symbol = match matched_symbol {
        MatchedSymbol::Bracket => Symbol::RightBracket,
        MatchedSymbol::Brace => Symbol::RightBrace,
        _ => unreachable!(),
    };
    let token = token.unwrap();
    if !matches!(token, Token {
        r#type: TokenType::Symbol(ref symbol_),
        ..
    } if *symbol_ == target_symbol) {
        return Err(UnmatchedSymbolError::new(matched_symbol, token.position));
    }
    return Ok(());
}
fn match_symbol(token: Option<Token>, symbol: Symbol, position: Position) -> ParseResult<Position> {
    if token.is_none() {
        return Err(UnexpectedEOFError::new(position));
    }
    let token = token.unwrap();
    if !matches!(token, Token {
        r#type: TokenType::Symbol(ref symbol_),
        ..
    } if *symbol_ == symbol) {
        return Err(UnexpectedTokenError::new(token));
    }
    return Ok(token.position);
}
fn match_key(token: Option<Token>, position: Position) -> ParseResult<String> {
    if token.is_none() {
        return Err(UnexpectedEOFError::new(position));
    }
    let token = token.unwrap();
    let Token {
        r#type: TokenType::String(key),
        ..
    } = token else {
        return Err(UnexpectedTokenError::new(token));
    };
    return Ok(key);
}
