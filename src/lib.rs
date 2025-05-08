pub mod elements;

use hex::FromHex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::ops::Add;
use std::str::FromStr;
pub use xmltree;
use xmltree::{AttributeMap, Element, XMLNode};
pub fn gen_svg(node: XMLNode, view_box: (f32, f32)) -> Element {
    let mut top_element = Element::new("svg");
    top_element.attributes.insert(
        "viewBox".to_string(),
        format!("0 0 {} {}", view_box.0, view_box.1),
    );
    top_element.attributes.insert(
        "xmlns".to_string(),
        "http://www.w3.org/2000/svg".to_string(),
    );
    top_element.children.push(node);
    top_element
}

pub type UkkoResult<T> = Result<T, UkkoError>;

#[derive(Debug)]
pub enum UkkoError {
    ParseError(String),
    TODO,
}

pub struct SvgVec {
    x: SvgValue,
    y: SvgValue,
}

impl From<f32> for SvgValue {
    fn from(value: f32) -> Self {
        Self::Length(value)
    }
}

impl Into<f32> for SvgValue {
    fn into(self) -> f32 {
        match self {
            SvgValue::Length(v) | SvgValue::Percentage(v) => v,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SvgValue {
    Length(f32),
    Percentage(f32),
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

pub trait Attribute: Sized {
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

fn to_hex_int(val: f32) -> u8 {
    let val = val.min(1.).max(0.);
    (255. * val).floor() as u8
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Self::from_rgb(value[0], value[1], value[2])
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() < 6 || value.len() > 7 {
            return Err(());
        }
        if value.len() == 7 {
            if value.chars().next() != Some('#') {
                return Err(());
            }
            <[u8; 3]>::from_hex(&value[1..])
        } else {
            <[u8; 3]>::from_hex(&value)
        }
        .map_err(|_| ())
        .map(|v| v.into())
    }
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        [to_hex_int(self.r), to_hex_int(self.g), to_hex_int(self.b)]
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        let (r, g, b) = (
            (self.r + rhs.r).min(255.),
            (self.g + rhs.g).min(255.),
            (self.b + rhs.b).min(255.),
        );
        Self::new(r, g, b)
    }
}

impl Color {
    pub const RED: Color = Color {
        r: 1.,
        g: 0.,
        b: 0.,
    };
    pub const GREEN: Color = Color {
        r: 0.,
        g: 1.,
        b: 0.,
    };
    pub const BLUE: Color = Color {
        r: 0.,
        g: 0.,
        b: 1.,
    };

    pub const BLACK: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
    };
    pub const WHITE: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
    };
    pub const YELLOW: Color = Color {
        r: 1.,
        g: 1.,
        b: 0.,
    };
    pub const PURPLE: Color = Color {
        r: 1.,
        g: 0.,
        b: 1.,
    };
    pub const CYAN: Color = Color {
        r: 0.,
        g: 1.,
        b: 1.,
    };

    pub fn average(&self, rhs: Color) -> Self {
        let (r, g, b) = (
            (self.r + rhs.r) / 2.,
            (self.g + rhs.g) / 2.,
            (self.b + rhs.b) / 2.,
        );
        Self::new(r, g, b)
    }

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let (r, g, b) = ((r as f32) / 255., (g as f32) / 255., (b as f32) / 255.);
        Self::new(r, g, b)
    }

    pub fn to_hex(&self) -> (u8, u8, u8) {
        (to_hex_int(self.r), to_hex_int(self.g), to_hex_int(self.b))
    }

    pub fn to_hex_code(&self) -> String {
        let (r, g, b) = self.to_hex();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex_code())
    }
}

pub enum Fill {
    Color(Color),
    Url(String),
    Custom(String),
}

impl Attribute for Fill {
    fn key(&self) -> String {
        "fill".to_string()
    }

    fn value(&self) -> String {
        match self {
            Fill::Color(c) => c.to_string(),
            Fill::Url(s) => s.clone(),
            Fill::Custom(s) => s.clone(),
        }
    }

    fn from_key_value(kv: (String, String)) -> UkkoResult<Self> {
        if kv.0.to_ascii_lowercase() != "fill" {
            return Err(UkkoError::TODO);
        }
        Ok(Self::Custom(kv.1)) // TODO: maybe check what kind of things are happening here
    }
}
