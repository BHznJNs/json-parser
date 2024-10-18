#[derive(Debug)]
pub enum Symbol {
    LeftBracket , // [
    RightBracket, // ]
    LeftBrace   , // {
    RightBrace  , // }

    Comma       , // ,
    Colon       , // :
    DoubleQuote , // "
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '[' => Self::LeftBracket ,
            ']' => Self::RightBracket,
            '{' => Self::LeftBrace   ,
            '}' => Self::RightBrace  ,
            ',' => Self::Comma       ,
            ':' => Self::Colon       ,
            '"' => Self::DoubleQuote ,
            _ => unreachable!(),
        }
    }
}

pub trait IsSymbolCharacter {
    fn is_symbol(self) -> bool;
}
impl IsSymbolCharacter for char {
    fn is_symbol(self) -> bool {
        matches!(self, '[' | ']' | '{' | '}' | ',' | ':' | '"')
    }
}

pub trait IsSignCharacter {
    fn is_sign(self) -> bool;
}
impl IsSignCharacter for char {
    fn is_sign(self) -> bool {
        self == '+' || self == '-'
    }
}
