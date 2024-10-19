use std::{iter::Peekable, str::Chars};

use crate::{public::position::Position, tokenizer::symbol::IsSignCharacter, Number, ParseResult, UnexpectedCharError};

use super::ResolvedPair;

pub fn resolve_number(iter: &mut Peekable<Chars>, position: &mut Position) -> ParseResult<ResolvedPair<Number>> {    
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