mod constant;
mod symbol;
mod token;

use std::{char, iter::Peekable, str::Chars};

use symbol::IsSignCharacter;
pub use token::{Token, TokenType};
pub use constant::Constant;
pub use symbol::IsSymbolCharacter;

use crate::{public::{
    position::Position,
    result::{ParseResult, UnexpectedCharError}
}, Number, UnexpectedEscapeCharError, UnmatchedQuoteError};

type ResolvedPair<T> = (String, T);

fn resolve_number(iter: &mut Peekable<Chars>, position: &mut Position) -> ParseResult<ResolvedPair<Number>> {    
    enum ResolveState {
        Int,
        Float,
        IExp,
        FExp,
    }
    enum ResultSign {
        Positive,
        Negative,
    }

    let mut state = ResolveState::Int;
    let mut result_int: i64 = 0;
    let mut result_float: f64 = 0.0;
    let mut decimal_places: f64 = 10.0;
    let mut result_exp: i32 = 0;
    let mut result_sign = ResultSign::Positive;
    let mut lexeme = String::new();

    if iter.peek().is_some_and(|ch| ch.is_sign()) {
        let sign_ch = iter.next().unwrap();
        lexeme.push(sign_ch);
        position.incr_col();

        let is_positive = sign_ch == '+';
        if !is_positive {
            result_sign = ResultSign::Negative;
        }
    }

    while let Some(&ch) = iter.peek() {
        let is_numeric = ch.is_numeric();
        let is_dot = ch == '.';
        let is_exp = ch == 'e' || ch == 'E';

        if !is_numeric && !is_dot && !is_exp {
            break;
        }
        if is_numeric {
            let temp_number: u32 = ch.to_digit(10).unwrap().into();
            match state {
                ResolveState::Int => result_int = result_int * 10 + temp_number as i64,
                ResolveState::Float => {
                    result_float = result_float + temp_number as f64 / decimal_places;
                    decimal_places *= 10.0;
                }
                ResolveState::IExp | ResolveState::FExp => result_exp = result_exp * 10 + temp_number as i32,
            };
        }
        if is_dot {
            if let ResolveState::Float = state {
                return Err(UnexpectedCharError::new('.', *position));
            }
            state = ResolveState::Float;
            result_float = result_int as f64;
        }
        if is_exp {
            match state {
                ResolveState::Int => state = ResolveState::IExp,
                ResolveState::Float => state = ResolveState::FExp,
                _ => return Err(UnexpectedCharError::new('e', *position)),
            }

            if iter.peek().is_some_and(|ch| ch.is_sign()) {
                let sign_ch = iter.next().unwrap();
                lexeme.push(sign_ch);
                position.incr_col();

                let is_positive = sign_ch == '+';
                if !is_positive {
                    result_exp = -result_exp;
                }
            }
        }

        lexeme.push(ch);
        iter.next().unwrap();
        position.incr_col();
    }

    if let ResultSign::Negative = result_sign {
        result_int = -result_int;
        result_float = -result_float;
    }

    let result = match state {
        ResolveState::Int => (lexeme, Number::Int(result_int)),
        ResolveState::Float => (lexeme, Number::Float(result_float)),
        ResolveState::IExp => (lexeme, Number::Int(result_int.pow(result_exp as u32))),
        ResolveState::FExp => (lexeme, Number::Float(result_float.powi(result_exp as i32))),
    };
    return Ok(result);
}

fn resolve_string(iter: &mut Peekable<Chars>, position: &mut Position) -> ParseResult<ResolvedPair<String>> {
    fn try_to_escape(ch: char) -> Option<char> {
        let result = match ch {
            '"' => '\"',
            'b' => '\x08',
            'f' => '\x0C',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            '/' => '/',
            '\\' => '\\',
            _ => return None,
        };
        return Some(result);
    }
    
    let mut result_string = String::new();
    let mut lexeme = String::new();
    
    if iter.peek().is_some_and(|&ch| ch == '"') {
        iter.next().unwrap();
        lexeme.push('"');
        position.incr_col();
    }

    let mut is_escaped_char = false;
    let mut has_end_quote = false;

    while let Some(&ch) = iter.peek() {
        if ch == '"' && !is_escaped_char {
            iter.next().unwrap();
            lexeme.push(ch);
            position.incr_col();
            has_end_quote = true;
            break;
        }

        if ch == '\\' && !is_escaped_char {
            is_escaped_char = true;
        } else if is_escaped_char {
            let Some(escape_ch) = try_to_escape(ch) else {
                return Err(UnexpectedEscapeCharError::new(ch, *position));
            };
            is_escaped_char = false;
            result_string.push(escape_ch);
        } else {
            result_string.push(ch);
        }
        iter.next().unwrap();
        lexeme.push(ch);
        position.incr_col();
    }

    if !has_end_quote {
        return Err(UnmatchedQuoteError::new(lexeme, *position));
    }
    return Ok((lexeme, result_string));
}

fn resolve_identifier(iter: &mut Peekable<Chars>, position: &mut Position) -> ParseResult<ResolvedPair<Constant>> {
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

pub fn tokenize(source: &str) -> ParseResult<Vec<Token>> {
    let mut tokens = vec![];
    let mut position = Position::new(0, 0);

    for line in source.lines() {
        let mut char_iter = line.chars().peekable();
        while let Some(&ch) = char_iter.peek() {
            if matches!(ch, ' ' | '\t') {
                char_iter.next().unwrap();
                position.incr_col();
                continue;
            }
            if ch.is_numeric() || ch == '+' || ch == '-' {
                let (lexeme, number_val) = resolve_number(&mut char_iter, &mut position)?;
                tokens.push(Token {
                    r#type: TokenType::Number(number_val),
                    lexeme,
                    position,
                });
                position.incr_col();
                continue;
            }
            if ch.is_ascii_alphabetic() {
                let (lexeme, constant_val) = resolve_identifier(&mut char_iter, &mut position)?;
                tokens.push(Token {
                    r#type: TokenType::Constant(constant_val),
                    lexeme,
                    position,
                });
                continue;
            }
            if ch.is_symbol() {
                if ch == '"' {
                    // start string resolving
                    let (lexeme, string_val) = resolve_string(&mut char_iter, &mut position)?;
                    tokens.push(Token {
                        r#type: TokenType::String(string_val),
                        lexeme,
                        position,
                    });
                } else {
                    todo!();
                    position.incr_col();
                }
                continue;
            }

            return Err(UnexpectedCharError::new(ch, position));
        }

        position.incr_row();
    }
    return Ok(tokens);
}
