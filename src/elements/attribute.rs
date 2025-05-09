use crate::elements::value::color::CssColor;
use crate::elements::value::display::DisplayOutsideInside;
use crate::elements::value::display::{
    DisplayBox, DisplayInternal, DisplayLegacy, DisplayListItem,
};
use crate::elements::value::{
    BasicShape, BeginEndValue, ClockValue, GeometryBox, Length, LengthPercentage,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub enum Attribute {}

fn concat_str_list<T: Display>(input: &[T], separator: &str) -> String {
    input
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(separator)
}

pub trait FloatAttr {
    fn float_value(&self) -> f64;

    fn rounded_value(&self, precision: Option<usize>) -> String {
        if let Some(precision) = precision {
            format!("{:.*}", precision, self.float_value())
        } else {
            self.float_value().to_string()
        }
    }
}

pub trait EnumAttr {}

pub trait StringAttr {}

pub trait NumberOptionalNumberAttr {
    fn float_value(&self) -> f64;

    fn optional_value(&self) -> Option<f64>;

    fn value_rounded(&self, precision: Option<usize>) -> String {
        match (self.optional_value(), precision) {
            (Some(o), Some(p)) => {
                format!("{:.*} {:.*}", p, self.float_value(), p, o)
            }
            (Some(o), _) => {
                format!("{} {}", self.float_value(), o)
            }
            (_, Some(p)) => {
                format!("{:.*}", p, self.float_value())
            }
            _ => self.float_value().to_string(),
        }
    }
}

pub trait SpecialAttr {}

pub trait Attr {
    fn name(&self) -> String;
    fn value(&self) -> String;

    fn valid_elements(&self) -> Vec<()> {
        vec![]
    }

    fn is_limited(&self) -> bool {
        false
    }

    fn name_value(&self) -> String {
        format!("{}=\"{}\"", self.name(), self.value())
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum Accumulate {
    #[default]
    None,
    Sum,
}

impl EnumAttr for Accumulate {}

impl Attr for Accumulate {
    fn name(&self) -> String {
        "Accumulate".to_string()
    }

    fn value(&self) -> String {
        match self {
            Accumulate::None => "none".to_string(),
            Accumulate::Sum => "sum".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum Additive {
    #[default]
    Replace,
    Sum,
}

impl EnumAttr for Additive {}

impl Attr for Additive {
    fn name(&self) -> String {
        "Additive".to_string()
    }

    fn value(&self) -> String {
        match self {
            Additive::Replace => "replace".to_string(),
            Additive::Sum => "sum".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum AlignmentBaseline {
    #[default]
    Auto,
    Baseline,
    BeforeEdge,
    TextBeforeEdge,
    Middle,
    Central,
    AfterEdge,
    TextAfterEdge,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
    Top,
    Center,
    Bottom,
}

impl EnumAttr for AlignmentBaseline {}

impl Attr for AlignmentBaseline {
    fn name(&self) -> String {
        "alignment-baseline".to_string()
    }

    fn value(&self) -> String {
        match self {
            AlignmentBaseline::Auto => "auto".to_string(),
            AlignmentBaseline::Baseline => "baseline".to_string(),
            AlignmentBaseline::BeforeEdge => "before-edge".to_string(),
            AlignmentBaseline::TextBeforeEdge => "text-before-edge".to_string(),
            AlignmentBaseline::Middle => "middle".to_string(),
            AlignmentBaseline::Central => "central".to_string(),
            AlignmentBaseline::AfterEdge => "after-edge".to_string(),
            AlignmentBaseline::TextAfterEdge => "text-after-edge".to_string(),
            AlignmentBaseline::Ideographic => "ideographic".to_string(),
            AlignmentBaseline::Alphabetic => "alphabetic".to_string(),
            AlignmentBaseline::Hanging => "hanging".to_string(),
            AlignmentBaseline::Mathematical => "mathematical".to_string(),
            AlignmentBaseline::Top => "top".to_string(),
            AlignmentBaseline::Center => "center".to_string(),
            AlignmentBaseline::Bottom => "bottom".to_string(),
        }
    }

    fn is_limited(&self) -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Amplitude(pub f64);

impl Default for Amplitude {
    fn default() -> Self {
        Amplitude(1.0)
    }
}

impl FloatAttr for Amplitude {
    fn float_value(&self) -> f64 {
        self.0
    }
}

impl Attr for Amplitude {
    fn name(&self) -> String {
        "amplitude".to_string()
    }

    fn value(&self) -> String {
        format!("{}", self.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttributeName(pub String);

impl StringAttr for AttributeName {}

impl Attr for AttributeName {
    fn name(&self) -> String {
        "attributeName".to_string()
    }

    fn value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[deprecated(since = "0.1.0", note = "Deprecated svg attribute.")]
pub enum AttributeType {
    CSS,
    XML,
    #[default]
    Auto,
}

impl EnumAttr for AttributeType {}

impl Attr for AttributeType {
    fn name(&self) -> String {
        "attributeType".to_string()
    }

    fn value(&self) -> String {
        match self {
            AttributeType::CSS => "CSS".to_string(),
            AttributeType::XML => "XML".to_string(),
            AttributeType::Auto => "auto".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Azimuth(pub f64);

impl FloatAttr for Azimuth {
    fn float_value(&self) -> f64 {
        self.0
    }
}

impl Attr for Azimuth {
    fn name(&self) -> String {
        "azimuth".to_string()
    }

    fn value(&self) -> String {
        self.rounded_value(None)
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct BaseFrequency(pub f64, pub Option<f64>);

impl NumberOptionalNumberAttr for BaseFrequency {
    fn float_value(&self) -> f64 {
        self.0
    }

    fn optional_value(&self) -> Option<f64> {
        self.1
    }
}

impl Attr for BaseFrequency {
    fn name(&self) -> String {
        "baseFrequency".to_string()
    }

    fn value(&self) -> String {
        self.value_rounded(None)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BaselineShift {
    LengthPercentage(f64),
    Sub,
    Super,
}

impl Default for BaselineShift {
    fn default() -> Self {
        Self::LengthPercentage(0.0)
    }
}

impl EnumAttr for BaselineShift {}

impl Attr for BaselineShift {
    fn name(&self) -> String {
        "baseline-shift".to_string()
    }

    fn value(&self) -> String {
        match self {
            BaselineShift::LengthPercentage(v) => v.to_string(),
            BaselineShift::Sub => "sub".to_string(),
            BaselineShift::Super => "super".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[deprecated(since = "0.1.0", note = "Deprecated svg attribute.")]
pub struct BaseProfile(pub String);

impl StringAttr for BaseProfile {}

impl Attr for BaseProfile {
    fn name(&self) -> String {
        "baseProfile".to_string()
    }

    fn value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Begin(pub Vec<BeginEndValue>);

impl Attr for Begin {
    fn name(&self) -> String {
        "begin".to_string()
    }

    fn value(&self) -> String {
        self.0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(";")
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Bias(pub f64);

impl FloatAttr for Bias {
    fn float_value(&self) -> f64 {
        self.0
    }
}

impl Attr for Bias {
    fn name(&self) -> String {
        "bias".to_string()
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct By(pub String);

impl SpecialAttr for By {}

impl Attr for By {
    fn name(&self) -> String {
        "by".to_string()
    }

    fn value(&self) -> String {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum CalcMode {
    Discrete,
    #[default]
    Linear,
    Paced,
    Spline,
}

impl EnumAttr for CalcMode {}

impl Attr for CalcMode {
    fn name(&self) -> String {
        "calcMode".to_string()
    }

    fn value(&self) -> String {
        match self {
            CalcMode::Discrete => "discrete",
            CalcMode::Linear => "linear",
            CalcMode::Paced => "paced",
            CalcMode::Spline => "spline",
        }
        .to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Class(Vec<String>);

impl SpecialAttr for Class {}

impl Attr for Class {
    fn name(&self) -> String {
        "class".to_string()
    }

    fn value(&self) -> String {
        self.0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[deprecated(since = "0.1.0", note = "Deprecated svg attribute.")]
pub enum Clip {
    #[default]
    Auto,
    Rect(Length, Length, Length, Length),
}

impl EnumAttr for Clip {}

impl Attr for Clip {
    fn name(&self) -> String {
        "clip".to_string()
    }

    fn value(&self) -> String {
        match self {
            Clip::Auto => "auto".to_string(),
            Clip::Rect(a, b, c, d) => {
                format!("rect({},{},{},{})", a, b, c, d)
            }
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum ClipPath {
    Url(String),
    BasicShape(BasicShape),
    GeometryBox(GeometryBox),
    BasicShapeGeometryBox(BasicShape, GeometryBox),
    #[default]
    None,
}

impl EnumAttr for ClipPath {}

impl Attr for ClipPath {
    fn name(&self) -> String {
        "clip-path".to_string()
    }

    fn value(&self) -> String {
        match self {
            ClipPath::Url(url) => {
                format!("url({})", url)
            }
            ClipPath::BasicShape(bs) => bs.to_string(),
            ClipPath::GeometryBox(gb) => gb.to_string(),
            ClipPath::BasicShapeGeometryBox(bs, gb) => {
                format!("{} {}", bs, gb)
            }
            ClipPath::None => "none".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum ClipRule {
    #[default]
    NonZero,
    EvenOdd,
    Inherit,
}

impl EnumAttr for ClipRule {}

impl Attr for ClipRule {
    fn name(&self) -> String {
        "clip-rule".to_string()
    }

    fn value(&self) -> String {
        match self {
            ClipRule::NonZero => "nonzero",
            ClipRule::EvenOdd => "evenodd",
            ClipRule::Inherit => "inherit",
        }
        .to_string()
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum ClipPathUnits {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl EnumAttr for ClipPathUnits {}

impl Attr for ClipPathUnits {
    fn name(&self) -> String {
        "clipPathUnits".to_string()
    }

    fn value(&self) -> String {
        match self {
            ClipPathUnits::UserSpaceOnUse => "userSpaceOnUse",
            ClipPathUnits::ObjectBoundingBox => "objectBoundingBox",
        }
        .to_string()
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum Color {
    Color(CssColor),
    #[default]
    Inherit,
}

impl Attr for Color {
    fn name(&self) -> String {
        "color".to_string()
    }

    fn value(&self) -> String {
        match self {
            Color::Color(c) => c.to_string(),
            Color::Inherit => "inherit".to_string(),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub enum ColorInterpolation {
    Auto,
    #[default]
    Srgb,
    LinearRgb,
}

impl Attr for ColorInterpolation {
    fn name(&self) -> String {
        "color-interpolation".to_string()
    }

    fn value(&self) -> String {
        match self {
            ColorInterpolation::Auto => "auto",
            ColorInterpolation::Srgb => "sRGB",
            ColorInterpolation::LinearRgb => "linearRGB",
        }
        .to_string()
    }
}
#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub enum ColorInterpolationFilter {
    Auto,
    Srgb,
    #[default]
    LinearRgb,
}

impl Attr for ColorInterpolationFilter {
    fn name(&self) -> String {
        "color-interpolation-filter".to_string()
    }

    fn value(&self) -> String {
        match self {
            ColorInterpolationFilter::Auto => "auto",
            ColorInterpolationFilter::Srgb => "sRGB",
            ColorInterpolationFilter::LinearRgb => "linearRGB",
        }
        .to_string()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum CursorType {
    Auto,
    Crosshair,
    Default,
    Pointer,
    Move,
    EResize,
    NeResize,
    NwResize,
    NResize,
    SeResize,
    SwResize,
    SResize,
    WResize,
    Text,
    Wait,
    Help,
}

impl Display for CursorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CursorType::Auto => {
                    "auto"
                }
                CursorType::Crosshair => {
                    "crosshair"
                }
                CursorType::Default => {
                    "default"
                }
                CursorType::Pointer => {
                    "pointer"
                }
                CursorType::Move => {
                    "move"
                }
                CursorType::EResize => {
                    "e-resize"
                }
                CursorType::NeResize => {
                    "ne-resize"
                }
                CursorType::NwResize => {
                    "nw-resize"
                }
                CursorType::NResize => {
                    "n-resize"
                }
                CursorType::SeResize => {
                    "se-resize"
                }
                CursorType::SwResize => {
                    "sw-resize"
                }
                CursorType::SResize => {
                    "s-resize"
                }
                CursorType::WResize => {
                    "w-resize"
                }
                CursorType::Text => {
                    "text"
                }
                CursorType::Wait => {
                    "wait"
                }
                CursorType::Help => {
                    "help"
                }
            }
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComplexCursor(Vec<String>, CursorType);

impl Display for ComplexCursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut urls = self
            .0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");
        if self.0.len() > 0 {
            urls.push(',')
        }
        write!(f, "{}{}", urls, self.1)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Cursor {
    Complex(ComplexCursor),
    Inherit,
}

impl Attr for Cursor {
    fn name(&self) -> String {
        "cursor".to_string()
    }

    fn value(&self) -> String {
        match self {
            Cursor::Complex(cc) => cc.to_string(),
            Cursor::Inherit => "inherit".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Cx(LengthPercentage);

impl Attr for Cx {
    fn name(&self) -> String {
        "cx".to_string()
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Cy(LengthPercentage);

impl Attr for Cy {
    fn name(&self) -> String {
        "cy".to_string()
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct D(pub String); // TODO: Path

impl Attr for D {
    fn name(&self) -> String {
        "d".to_string()
    }

    fn value(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data(pub String, pub String);

impl Attr for Data {
    fn name(&self) -> String {
        format!("data-{}", self.0)
    }

    fn value(&self) -> String {
        self.1.clone()
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum Decoding {
    Sync,
    Async,
    #[default]
    Auto,
}

impl Attr for Decoding {
    fn name(&self) -> String {
        "decoding".to_string()
    }

    fn value(&self) -> String {
        use Decoding::*;
        match self {
            Sync => "sync",
            Async => "async",
            Auto => "auto",
        }
        .to_string()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DiffuseConstant(pub f64);

impl Default for DiffuseConstant {
    fn default() -> Self {
        DiffuseConstant(1.)
    }
}

impl Attr for DiffuseConstant {
    fn name(&self) -> String {
        "diffuseConstant".to_string()
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl Attr for Direction {
    fn name(&self) -> String {
        "direction".to_string()
    }

    fn value(&self) -> String {
        use Direction::*;
        match self {
            Ltr => "ltr",
            Rtl => "rtl",
        }
        .to_string()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DisplayA {
    OutsideInside(DisplayOutsideInside),
    ListItem(DisplayListItem),
    Internal(DisplayInternal),
    Box(DisplayBox),
    Legacy(DisplayLegacy),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Divisor(pub f64);

impl Attr for Divisor {
    fn name(&self) -> String {
        "divisor".to_string()
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum DominantBaseline {
    #[default]
    Auto,
    TextBottom,
    Alphabetic,
    Ideographic,
    Middle,
    Central,
    Mathematical,
    Hanging,
    TextTop,
}

impl Attr for DominantBaseline {
    fn name(&self) -> String {
        "dominant-baseline".to_string()
    }

    fn value(&self) -> String {
        match self {
            DominantBaseline::Auto => "auto",
            DominantBaseline::TextBottom => "text-bottom",
            DominantBaseline::Alphabetic => "alphabetic",
            DominantBaseline::Ideographic => "ideographic",
            DominantBaseline::Middle => "middle",
            DominantBaseline::Central => "central",
            DominantBaseline::Mathematical => "mathematical",
            DominantBaseline::Hanging => "hanging",
            DominantBaseline::TextTop => "text-top",
        }
        .to_string()
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub enum Dur {
    ClockValue(ClockValue),
    Media,
    #[default]
    Indefinite,
}

impl Attr for Dur {
    fn name(&self) -> String {
        "dur".to_string()
    }

    fn value(&self) -> String {
        match self {
            Dur::ClockValue(cv) => cv.to_string(),
            Dur::Media => "media".to_string(),
            Dur::Indefinite => "indefinite".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Dx {
    Number(f64),
    List(Vec<LengthPercentage>),
}

impl Attr for Dx {
    fn name(&self) -> String {
        "dx".to_string()
    }

    //noinspection DuplicatedCode
    fn value(&self) -> String {
        match self {
            Dx::Number(n) => n.to_string(),
            Dx::List(l) => concat_str_list(&l, " "),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Dy {
    Number(f64),
    List(Vec<LengthPercentage>),
}

impl Attr for Dy {
    fn name(&self) -> String {
        "dy".to_string()
    }

    //noinspection DuplicatedCode
    fn value(&self) -> String {
        match self {
            Dy::Number(n) => n.to_string(),
            Dy::List(l) => concat_str_list(&l, " "),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum EdgeMode {
    Duplicate,
    Wrap,
    None,
}

impl Attr for EdgeMode {
    fn name(&self) -> String {
        "edgeMode".to_string()
    }

    fn value(&self) -> String {
        match self {
            EdgeMode::Duplicate => "duplicate",
            EdgeMode::Wrap => "wrap",
            EdgeMode::None => "none",
        }
        .to_string()
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Elevation(pub f64);

impl Attr for Elevation {
    fn name(&self) -> String {
        "elevation".to_string()
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct End(pub Vec<BeginEndValue>);

impl Attr for End {
    fn name(&self) -> String {
        "end".to_string()
    }

    fn value(&self) -> String {
        concat_str_list(&self.0, ";")
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Exponent(pub f64);

impl Attr for Exponent {
    fn name(&self) -> String {
        "exponent".to_string()
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}
