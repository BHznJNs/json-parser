use std::{iter::Peekable, str::Chars};

use crate::{public::position::Position, ParseResult, UnexpectedEscapeCharError, UnmatchedQuoteError};

use super::ResolvedPair;

pub fn resolve_string(iter: &mut Peekable<Chars>, position: &mut Position) -> ParseResult<ResolvedPair<String>> {
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