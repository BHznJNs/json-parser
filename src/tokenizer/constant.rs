use crate::IntoValue;

#[derive(Clone, Copy, Debug)]
pub enum Constant {
    True,
    False,
    Null,
}

impl TryFrom<String> for Constant {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let ret = match value.as_str() {
            "true" => Self::True,
            "false" => Self::False,
            "null" => Self::Null,
            _ => return Err(value),
        };
        return Ok(ret);
    }
}

impl IntoValue for Constant {
    fn into_val(self) -> crate::Value {
        match self {
            Constant::True => crate::Value::Bool(true),
            Constant::False => crate::Value::Bool(false),
            Constant::Null => crate::Value::Null,
        }
    }
}

pub enum ConstantExtended {
    True,
    False,
    Null,
    NaN,
    Infinify,
}

impl TryFrom<String> for ConstantExtended {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let ret = match value.as_str() {
            "true" => Self::True,
            "false" => Self::False,
            "null" => Self::Null,
            "NaN" => Self::NaN,
            "Infinify" => Self::Infinify,
            _ => return Err(value),
        };
        return Ok(ret);
    }
}
