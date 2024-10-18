use super::IntoValue;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    Float(f64),
    Int(i64),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(float_val) => write!(f, "{}", float_val),
            Self::Int(int_val) => write!(f, "{}", int_val),
        }
    }
}

impl IntoValue for Number {
    fn into_val(self) -> super::Value {
        super::Value::Number(self)
    }
}
