mod public;
mod tokenizer;
mod deserializer;

pub use public::result::*;
pub use public::value::*;
pub use struct_field_derive::ListedFieldNames;

pub trait ListedFieldNames {
    fn fields() -> Vec<&'static str>;
}

pub fn parse(source: &str) -> ParseResult<Value> {
    let tokens = tokenizer::tokenize(source)?;
    let result = deserializer::deserialize(tokens)?;
    return Ok(result);
}
