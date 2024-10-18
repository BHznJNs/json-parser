mod number;

use std::collections::HashMap;
use std::fmt;

pub use number::Number;

pub type Object = HashMap<String, Box<Value>>;
pub type Array  = Vec<Value>;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Object(Object),
    Array (Array),
    String(String),
    Number(Number),
    Bool(bool),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn object_display_helper(obj: &Object) -> String {
            let mut buf = String::new();
            buf.push('{');

            let mut index = 0;
            for (k, v) in obj {
                buf.push_str(k);
                buf.push(':');
                buf.push_str(&v.to_string());

                if index < obj.len() {
                    buf.push(',');
                }
                index += 1;
            }

            buf.push('}');
            return buf;
        }
        fn array_display_helper(arr: &Array) -> String {
            let mut buf = String::new();
            buf.push('[');

            let mut index = 0;
            for item in arr {
                buf.push_str(&item.to_string());

                if index < arr.len() {
                    buf.push(',');
                }
                index += 1;
            }

            buf.push(']');
            return buf;
        }

        match self {
            Self::Null => write!(f, "null")?,
            Self::Bool(bool_val) => write!(f, "{}", bool_val)?,
            Self::Number(number_val) => write!(f, "{}", number_val)?,
            Self::String(string_val) => write!(f, "{}", string_val)?,

            Self::Object(obj) => write!(f, "{}", object_display_helper(obj))?,
            Self::Array(arr) => write!(f, "{}", array_display_helper(arr))?,
        }
        return Ok(());
    }
}

pub trait IntoValue {
    fn into_val(self) -> Value;
}

impl IntoValue for String {
    fn into_val(self) -> Value {
        Value::String(self)
    }
}
