use std::iter::Map;

mod number;
use number::Number;

#[derive(Clone)]
pub enum Value {
    Object(Map<String, Box<Value>>),
    Array(Vec<Value>),
    String(String),
    Number(Number),
    Bool(bool),
    Null,
}
