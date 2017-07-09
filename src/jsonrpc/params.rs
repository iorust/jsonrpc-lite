use {Value, Map};

/// A Structured value that holds the parameter values
/// to be used during the invocation of the method.
/// This member MAY be omitted.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Params {
    /// The Array Variant of a possible JSON-RPC Paramater
    Array(Vec<Value>),
    /// The Object Variant of a possible JSON-RPC Paramater
    Map(Map<String, Value>),
    /// The None Variant of a possible JSON-RPC Paramater
    None,
}

impl Params {
    /// Returns the parameters as an Array if they were an Array
    /// ## Examples
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #
    /// # fn main() {
    /// use jsonrpc_lite::Params;
    ///
    /// let x: Params = vec![42,42,42].into();
    /// assert!(x.get_array().is_some());
    /// # }
    /// ```
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #
    /// # fn main() {
    /// use jsonrpc_lite::Params;
    ///
    /// let x: Params = ().into();
    /// assert!(x.get_array().is_none());
    /// # }
    /// ```
    pub fn get_array(&self) -> Option<&Vec<Value>> {
        if let Params::Array(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the parameters as a [Map][map] (with string keys),
    /// if the `Params` were a JSON Object.
    /// ## Examples
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #
    /// # fn main() {
    /// use jsonrpc_lite::{Params, Map};
    ///
    /// let mut m = Map::new();
    /// m.insert("Lorem".to_string(), "ipsum".into());
    /// let x: Params = m.into();
    /// assert!(x.get_map().is_some());
    /// # }
    /// ```
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #
    /// # fn main() {
    /// use jsonrpc_lite::Params;
    ///
    /// let x: Params = ().into();
    /// assert!(x.get_map().is_none());
    /// # }
    /// ```
    ///
    /// [map]: ../serde_json/map/struct.Map.html
    pub fn get_map(&self) -> Option<&Map<String, Value>> {
        if let Params::Map(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }
}

impl<T: Into<Value>> From<Vec<T>> for Params {
    /// Convert a `Vec` to `Params`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #
    /// # fn main() {
    /// use jsonrpc_lite::Params;
    ///
    /// let v = vec!["lorem", "ipsum", "dolor"];
    /// let x: Params = v.into();
    /// # }
    /// ```
    fn from(val: Vec<T>) -> Self {
        Params::Array(val.into_iter().map(Into::into).collect())
    }
}

impl<'a, T: Clone + Into<Value>> From<&'a [T]> for Params {
    /// Convert a slice to `Params`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #
    /// # fn main() {
    /// use jsonrpc_lite::Params;
    ///
    /// let v: &[&str] = &["lorem", "ipsum", "dolor"];
    /// let x: Params = v.into();
    /// # }
    /// ```
    fn from(f: &'a [T]) -> Self {
        Params::Array(f.into_iter().cloned().map(Into::into).collect())
    }
}

impl<T: Into<Value>> ::std::iter::FromIterator<T> for Params {
    /// Convert an iteratable type to a `Params`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #
    /// # fn main() {
    /// use jsonrpc_lite::Params;
    ///
    /// let v = std::iter::repeat(42).take(5);
    /// let x: Params = v.collect();
    /// # }
    /// ```
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #
    /// # fn main() {
    /// use jsonrpc_lite::Params;
    ///
    /// let v: Vec<_> = vec!["lorem", "ipsum", "dolor"];
    /// let x: Params = v.into_iter().collect();
    /// # }
    /// ```
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #
    /// # fn main() {
    /// use std::iter::FromIterator;
    /// use jsonrpc_lite::Params;
    ///
    /// let x: Params = Params::from_iter(vec!["lorem", "ipsum", "dolor"]);
    /// # }
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Params::Array(iter.into_iter().map(|x| x.into()).collect::<Vec<Value>>())
    }
}

impl From<Value> for Params {
    /// Convert a [`Value`][value] to `Params`.
    ///
    /// This is mostly for convenience when using the
    /// `json!` macro from `serde-json`.
    ///
    /// **Note** that due to that, the only valid JSON
    /// inputs to this are `Value::Map` and `Value::Array`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #[macro_use]
    /// extern crate serde_json;
    ///
    /// # fn main() {
    /// use jsonrpc_lite::Params;
    ///
    /// let x: Params = json!({"anObject": "test"}).into();
    /// let y: Params = json!(["test", "of", "JSON", "Array"]).into();
    /// // Other uses such as: json!(5), json!(true), etc.
    /// // Result in Params::None
    /// # }
    /// ```
    ///
    /// [value]: ../serde_json/value/enum.Value.html
    fn from(val: Value) -> Self {
        use Value::*;

        match val {
            Array(v) => Params::Array(v),
            Object(v) => Params::Map(v),
            _ => Params::None,
        }
    }
}

impl From<Map<String, Value>> for Params {
    /// Convert [Map][map] (with string keys) to `Params`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #
    /// # fn main() {
    /// use jsonrpc_lite::{Map, Params};
    ///
    /// let mut m = Map::new();
    /// m.insert("Lorem".to_string(), "ipsum".into());
    /// let x: Params = m.into();
    /// # }
    /// ```
    ///
    /// [map]: ../serde_json/map/struct.Map.html
    fn from(val: Map<String, Value>) -> Self {
        Params::Map(val)
    }
}

impl From<()> for Params {
    /// Convert Unit to `Params`, this is the
    /// simplest way to create null value `Params`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate jsonrpc_lite;
    /// #
    /// # fn main() {
    /// use jsonrpc_lite::Params;
    ///
    /// let x: Params = ().into();
    /// # }
    /// ```
    fn from(_: ()) -> Self {
        Params::None
    }
}
