use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Definitions {
    #[serde(default)]
    pub modules: BTreeMap<String, Module>,
    #[serde(default)]
    pub functions: BTreeMap<String, Function>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Function {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub docs: String,
    #[serde(default)]
    pub manual_url: Option<String>,
    #[serde(default)]
    pub parameters: BTreeMap<String, Parameter>,
    #[serde(default)]
    pub constructors: BTreeMap<String, Constructor>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Module {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub docs: String,
    pub dimensions: u8,
    #[serde(default)]
    pub dimensions_in: Option<u8>,
    #[serde(default)]
    pub manual_url: Option<String>,
    #[serde(default)]
    pub operator: bool,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub parameters: BTreeMap<String, Parameter>,
    #[serde(default)]
    pub constructors: BTreeMap<String, Constructor>,
    #[serde(default)]
    pub imports: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub docs: String,
    #[serde(default)]
    pub default: Option<Value>,
    #[serde(rename = "type")]
    pub ty: Type,
    #[serde(default)]
    pub alt: Vec<Type>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Bool,
    Scalar,
    Scalar2,
    Scalar3,
    Length,
    Length2,
    Length3,
    Angle,
    Angle2,
    Angle3,
    FragmentResolution,
    String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
    Boolean(bool),
    Float(f64),
    Float2([f64; 2]),
    Float3([f64; 3]),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Boolean(v) => v.fmt(f),
            Value::Float(v) => v.fmt(f),
            Value::Float2([a, b]) => write!(f, "[{a}, {b}]"),
            Value::Float3([a, b, c]) => write!(f, "[{a}, {b}, {c}]"),
            Value::String(v) => write!(f, "{v:?}"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Constructor {
    #[serde(default)]
    pub docs: String,
    #[serde(default)]
    pub arguments: Vec<String>,
}
