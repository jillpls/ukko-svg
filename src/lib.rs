pub mod elements;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use xmltree::{AttributeMap, Element, XMLNode};

pub type UkkoResult<T> = Result<T, UkkoError>;

#[derive(Debug)]
pub enum UkkoError {
    ParseError(String),
    TODO,
}

impl UkkoError {
    pub fn parse<S: ToString>(str: S) -> Self {
        Self::ParseError(str.to_string())
    }
}

impl From<ParseFloatError> for UkkoError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseError(value.to_string())
    }
}

impl From<ParseIntError> for UkkoError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseError(value.to_string())
    }
}

pub trait SvgElement {
    fn children(&self) -> Vec<Box<dyn SvgElement>>;
    fn attributes(&self) -> HashMap<String, String>;
    fn attributes_mut(&mut self) -> &mut HashMap<String, String>;

    fn with_attribute(&mut self, key: String, value: String) {
        self.attributes_mut().insert(key, value);
    }
    fn name(&self) -> String;
    fn value(&self) -> Option<&String> {
        None
    }

    fn to_xml_node(&self) -> XMLNode {
        XMLNode::Element(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: self.name(),
            attributes: AttributeMap::from(self.attributes()),
            children: self
                .children()
                .iter()
                .map(|c| c.to_xml_node())
                .collect::<Vec<_>>(),
        })
    }
}

pub trait Attribute : Sized {
    fn key(&self) -> String;
    fn value(&self) -> String;

    fn fmt_str(&self) -> String {
        format!("{}=\"{}\"", self.key(), self.value())
    }

    fn from_key_value(kv: (String, String)) -> UkkoResult<Self>;
}

impl Attribute for (String, String) {
    fn key(&self) -> String {
        self.0.clone()
    }

    fn value(&self) -> String {
        self.1.clone()
    }

    fn from_key_value(kv: (String, String)) -> UkkoResult<Self> {
        Ok(kv)
    }
}

pub enum Color {
    String(String),
    Code(String)
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Color::String(s) => { s.clone() }
            Color::Code(c) => { c.clone() }
        };
        write!(f, "{}", s)
    }
}

pub enum Fill {
    Color(Color),
    Url(String),
    Custom(String)
}

impl Attribute for Fill {
    fn key(&self) -> String {
        "fill".to_string()
    }

    fn value(&self) -> String {
        match self {
            Fill::Color(c) => { c.to_string() }
            Fill::Url(s) => { s.clone()}
            Fill::Custom(s) => { s.clone() }
        }
    }

    fn from_key_value(kv: (String, String)) -> UkkoResult<Self> {
        if kv.0.to_ascii_lowercase() != "fill" { return Err(UkkoError::TODO)}
        Ok(Self::Custom(kv.1)) // TODO: maybe check what kind of things are happening here
    }
}
