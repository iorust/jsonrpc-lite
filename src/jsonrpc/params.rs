use {Value, Map};

/// A Structured value that holds the parameter values
/// to be used during the invocation of the method.
/// This member MAY be omitted.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Params {
    Array(Vec<Value>),
    Map(Map<String, Value>),
    None,
}
impl Params {
    pub fn get_array(&self) -> Option<&Vec<Value>> {
        if let Params::Array(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_map(&self) -> Option<&Map<String, Value>> {
        if let Params::Map(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<Value> for Params {
    fn from(val: Value) -> Self {
        use Value::*;

        match val {
            Array(v) => Params::Array(v),
            Object(v) => Params::Map(v),
            _ => Params::None,
        }
    }
}

impl From<Vec<Value>> for Params {
    fn from(val: Vec<Value>) -> Self {
        Params::Array(val)
    }
}

impl From<Map<String, Value>> for Params {
    fn from(val: Map<String, Value>) -> Self {
        Params::Map(val)
    }
}

impl From<()> for Params {
    fn from(_: ()) -> Self {
        Params::None
    }
}
