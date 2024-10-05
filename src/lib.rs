mod value;
mod tokenizer;
mod serializer;

pub trait ListedFieldNames {
    fn fields() -> Vec<&'static str>;
}

pub fn parse(input: &str) -> value::Value {
    todo!()
}
