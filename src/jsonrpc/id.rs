/// An identifier established by the Client that MUST contain a String, Number,
/// or NULL value if included. If it is not included it is assumed to be a notification.
/// The value SHOULD normally not be Null and Numbers SHOULD NOT contain fractional parts
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    Num(i64),
    Str(String),
    None,
}
impl Id {
    pub fn as_num(&self) -> Option<i64> {
        if let Id::Num(v) = *self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<String> {
        if let Id::Str(ref v) = *self {
            Some(v.clone())
        } else {
            None
        }
    }
}

impl From<i64> for Id {
    fn from(val: i64) -> Self {
        Id::Num(val)
    }
}

impl From<String> for Id {
    fn from(val: String) -> Self {
        Id::Str(val)
    }
}

impl<'a> From<&'a str> for Id {
    fn from(val: &str) -> Self {
        Id::Str(val.into())
    }
}

impl From<()> for Id {
    fn from(_: ()) -> Self {
        Id::None
    }
}
