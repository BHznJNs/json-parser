mod constant;
mod symbol;
mod token;
mod resolvers;

pub use symbol::Symbol;
pub use token::{Token, TokenType};
pub use constant::Constant;
pub use symbol::IsSymbolCharacter;

use crate::public::{
    position::Position,
    result::{ParseResult, UnexpectedCharError}
};

pub fn tokenize(source: &str) -> ParseResult<Vec<Token>> {
    let mut tokens = vec![];
    let mut position = Position::new(0, 0);

    for line in source.lines() {
        let mut char_iter = line.chars().peekable();
        while let Some(&ch) = char_iter.peek() {
            let cached_position = position;

            if matches!(ch, ' ' | '\t') {
                char_iter.next().unwrap();
                position.incr_col();
                continue;
            }
            if ch.is_numeric() || ch == '+' || ch == '-' {
                let (lexeme, number_val) = resolvers::resolve_number(&mut char_iter, &mut position)?;
                tokens.push(Token {
                    r#type: TokenType::Number(number_val),
                    lexeme,
                    position: cached_position,
                });
                position.incr_col();
                continue;
            }
            if ch.is_ascii_alphabetic() {
                let (lexeme, constant_val) = resolvers::resolve_identifier(&mut char_iter, &mut position)?;
                tokens.push(Token {
                    r#type: TokenType::Constant(constant_val),
                    lexeme,
                    position: cached_position,
                });
                continue;
            }
            if ch == '"' {
                let (lexeme, string_val) = resolvers::resolve_string(&mut char_iter, &mut position)?;
                tokens.push(Token {
                    r#type: TokenType::String(string_val),
                    lexeme,
                    position: cached_position,
                });
                continue;
            }
            if ch.is_symbol() {
                char_iter.next().unwrap();
                position.incr_col();
                tokens.push(Token {
                    r#type: TokenType::Symbol(Symbol::from(ch)),
                    lexeme: ch.to_string(),
                    position: cached_position,
                });
                continue;
            }
            return Err(UnexpectedCharError::new(ch, position));
        }
        position.incr_row();
    }
    return Ok(tokens);
}
