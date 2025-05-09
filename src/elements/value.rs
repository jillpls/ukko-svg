pub mod position;
pub mod color;
pub mod display;

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::time::Duration;
use time::OffsetDateTime;
use crate::elements::value::position::Position;

fn format_iso8601_extended(dt: OffsetDateTime) -> String {
    let format = time::format_description::parse(
        "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond][offset_hour sign:mandatory]:[offset_minute]"
    ).expect("Invalid format string");

    dt.format(&format).expect("Formatting failed")
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Length(f64, Option<LengthUnit>);

impl Display for Length {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.0,
            self.1.map(|v| v.to_string()).unwrap_or_default()
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum LengthUnit {
    Pixels,
    Points,
    Picas,
    Inches,
    QuarterMillimeters,
    Millimeters,
    Centimeters,
    FontSize,
    FontXSize,
    CharacterAdvance0,
    RootElementFontSize,
    ViewPort1PercentWidth,
    ViewPort1PercentHeight,
    ViewPort1PercentMin,
    ViewPort1PercentMax,
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LengthUnit::Pixels => {
                    "px"
                }
                LengthUnit::Points => {
                    "pt"
                }
                LengthUnit::Picas => {
                    "pc"
                }
                LengthUnit::Inches => {
                    "in"
                }
                LengthUnit::QuarterMillimeters => {
                    "Q"
                }
                LengthUnit::Millimeters => {
                    "mm"
                }
                LengthUnit::Centimeters => {
                    "cm"
                }
                LengthUnit::FontSize => {
                    "em"
                }
                LengthUnit::FontXSize => {
                    "ex"
                }
                LengthUnit::CharacterAdvance0 => {
                    "ch"
                }
                LengthUnit::RootElementFontSize => {
                    "rem"
                }
                LengthUnit::ViewPort1PercentWidth => {
                    "vw"
                }
                LengthUnit::ViewPort1PercentHeight => {
                    "vh"
                }
                LengthUnit::ViewPort1PercentMin => {
                    "vmin"
                }
                LengthUnit::ViewPort1PercentMax => {
                    "vmax"
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct ClockValue(Duration);

impl From<Duration> for ClockValue {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

impl Into<Duration> for ClockValue {
    fn into(self) -> Duration {
        self.0
    }
}

impl Display for ClockValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let first_part = format!(
            "{:02}:{:02}:{:02}",
            self.hours(),
            self.minutes(),
            self.seconds()
        );
        if self.milliseconds() > 0 {
            write!(f, "{}.{}", first_part, self.milliseconds())
        } else {
            write!(f, "{}", first_part)
        }
    }
}

impl ClockValue {
    fn hours(&self) -> u64 {
        self.0.as_secs() / 3600
    }

    fn minutes(&self) -> u64 {
        (self.0.as_secs() % 3600) / 60
    }

    fn seconds(&self) -> u64 {
        self.0.as_secs() % 60
    }

    fn milliseconds(&self) -> u32 {
        self.0.subsec_millis()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SignedClockValue(bool, ClockValue);

impl Display for SignedClockValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if self.0 { "+" } else { "-" }, self.1)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BeginEnd {
    Begin,
    End,
}

impl Display for BeginEnd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BeginEnd::Begin => {
                    "begin"
                }
                BeginEnd::End => {
                    "end"
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Focus,
    Blur,
    FocusIn,
    FocusOut,
    DOMActivate,
    AuxClick,
    Click,
    DblClick,
    MouseDown,
    MouseEnter,
    MouseLeave,
    MouseMove,
    MouseOut,
    MouseOver,
    MouseUp,
    Wheel,
    BeforeInput,
    Input,
    KeyDown,
    KeyUp,
    CompositionOnStart,
    CompositionOnUpdate,
    CompositionOnEnd,
    Load,
    Unload,
    Abort,
    Error,
    Select,
    Resize,
    Scroll,
    BeginEvent,
    EndEvent,
    RepeatEvent,
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Event::Focus => {
                    "focus"
                }
                Event::Blur => {
                    "blur"
                }
                Event::FocusIn => {
                    "focusin"
                }
                Event::FocusOut => {
                    "focusout"
                }
                Event::DOMActivate => {
                    "DOMActivate"
                }
                Event::AuxClick => {
                    "auxclick"
                }
                Event::Click => {
                    "click"
                }
                Event::DblClick => {
                    "dblclick"
                }
                Event::MouseDown => {
                    "mousedown"
                }
                Event::MouseEnter => {
                    "mouseenter"
                }
                Event::MouseLeave => {
                    "mouseleave"
                }
                Event::MouseMove => {
                    "mousemove"
                }
                Event::MouseOut => {
                    "mouseout"
                }
                Event::MouseOver => {
                    "mouseover"
                }
                Event::MouseUp => {
                    "mouseup"
                }
                Event::Wheel => {
                    "wheel"
                }
                Event::BeforeInput => {
                    "beforeinput"
                }
                Event::Input => {
                    "input"
                }
                Event::KeyDown => {
                    "keydown"
                }
                Event::KeyUp => {
                    "keyup"
                }
                Event::CompositionOnStart => {
                    "compositiononstart"
                }
                Event::CompositionOnUpdate => {
                    "compositiononupdate"
                }
                Event::CompositionOnEnd => {
                    "compositiononend"
                }
                Event::Load => {
                    "load"
                }
                Event::Unload => {
                    "unload"
                }
                Event::Abort => {
                    "abort"
                }
                Event::Error => {
                    "error"
                }
                Event::Select => {
                    "select"
                }
                Event::Resize => {
                    "resize"
                }
                Event::Scroll => {
                    "scroll"
                }
                Event::BeginEvent => {
                    "beginEvent"
                }
                Event::EndEvent => {
                    "endEvent"
                }
                Event::RepeatEvent => {
                    "repeatEvent"
                }
            }
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BeginValue {
    Offset(ClockValue),
    SyncBase(String, BeginEnd, Option<SignedClockValue>),
    Event(String, Event, Option<SignedClockValue>),
    Repeat(String, usize, Option<SignedClockValue>),
    AccessKey(char, Option<SignedClockValue>),
    WallclockSync(OffsetDateTime),
    Indefinite,
}

impl Display for BeginValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let optional_cv = match self {
            BeginValue::SyncBase(_, _, Some(cv))
            | BeginValue::Event(_, _, Some(cv))
            | BeginValue::Repeat(_, _, Some(cv))
            | BeginValue::AccessKey(_, Some(cv)) => cv.to_string(),
            _ => "".to_string(),
        };
        match self {
            BeginValue::Offset(cv) => {
                write!(f, "{}", cv)
            }
            BeginValue::SyncBase(id, be, _) => {
                write!(f, "{}.{}{}", id, be, optional_cv)
            }
            BeginValue::Event(id, ev, _) => {
                write!(f, "{}.{}{}", id, ev, optional_cv)
            }
            BeginValue::Repeat(id, count, _) => {
                write!(f, "{}.repeat({}){}", id, count, optional_cv)
            }
            BeginValue::AccessKey(key, _) => {
                write!(f, "accessKey({}){}", key, optional_cv)
            }
            BeginValue::WallclockSync(dt) => {
                write!(f, "wallclock({})", format_iso8601_extended(*dt))
            }
            BeginValue::Indefinite => {
                write!(f, "indefinite")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum LengthPercentage {
    Length(Length),
    Percentage(f64),
}

impl Display for LengthPercentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LengthPercentage::Length(l) => {
                    l.to_string()
                }
                LengthPercentage::Percentage(p) => {
                    format!("{}%", p * 100.)
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum LengthPercentageAuto {
    LengthPercentage(LengthPercentage),
    Auto,
}

impl Display for LengthPercentageAuto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthPercentageAuto::LengthPercentage(lp) => { write!(f, "{}", lp) }
            LengthPercentageAuto::Auto => { write!(f, "auto") }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct BorderRadius(LengthPercentage, Option<LengthPercentage>);

impl Display for BorderRadius {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1.map(|v| format!(" {}", v)).unwrap_or_default())
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BasicShapeRect {
    Inset(
        LengthPercentage,
        Option<LengthPercentage>,
        Option<LengthPercentage>,
        Option<LengthPercentage>,
        Option<BorderRadius>,
    ),
    Xywh(
        LengthPercentage,
        LengthPercentage,
        LengthPercentage,
        LengthPercentage,
        Option<BorderRadius>,
    ),
    Rect(
        LengthPercentageAuto,
       LengthPercentageAuto,
        LengthPercentageAuto,
        LengthPercentageAuto,
        Option<BorderRadius>,
    ),
}

impl Display for BasicShapeRect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BasicShapeRect::Inset(a, b, c, d, br) => {
                let mut result = a.to_string();

                if let Some(b_val) = b {
                    result.push(' ');
                    result.push_str(&b_val.to_string());

                    if let Some(c_val) = c {
                        result.push(' ');
                        result.push_str(&c_val.to_string());

                        if let Some(d_val) = d {
                            result.push(' ');
                            result.push_str(&d_val.to_string());
                        }
                    }
                }
                write!(f, "inset({}{})", result, br.map(|v| format!(" round {}", v)).unwrap_or_default())
            }
            BasicShapeRect::Xywh(x, y, w, h, br) => {
                write!(f, "xywh({} {} {} {}{})", x, y, w, h, br.map(|v| format!(" round {}", v)).unwrap_or_default())
            }
            BasicShapeRect::Rect(a, b, c, d, br) => {
                write!(f, "rect({} {} {} {}{})", a, b, c, d, br.map(|v| format!(" round {}", v)).unwrap_or_default())
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum RadialExtent {
    ClosestCorner,
    ClosestSide,
    FarthestCorner,
    FarthestSide,
}

impl Display for RadialExtent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RadialExtent::ClosestCorner => { "closest-corner" }
            RadialExtent::ClosestSide => { "closest-side" }
            RadialExtent::FarthestCorner => { "farthest-corner" }
            RadialExtent::FarthestSide => { "farthest-side" }
        })

    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum RadialSize {
    RadialExtent(RadialExtent),
    Length(Length),
    LengthPercentage(LengthPercentage),
}

impl Display for RadialSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RadialSize::RadialExtent(re) => { re.to_string() }
            RadialSize::Length(l) => { l.to_string() }
            RadialSize::LengthPercentage(lp) => { lp.to_string()}
        })
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum FillRule {
    NonZero,
    EvenOdd
}

impl Display for FillRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            FillRule::NonZero => { "nonzero"}
            FillRule::EvenOdd => { "evenodd" }
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BasicShape {
    BasicShapeRect(BasicShapeRect),
    Circle(RadialSize, Option<Position>),
    Ellipse(RadialSize, Option<Position>),
    Polygon(Option<FillRule>, Option<Length>, Option<Vec<(LengthPercentage, LengthPercentage)>>),
    Path(Option<FillRule>, String)
}

impl Display for BasicShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BasicShape::BasicShapeRect(bsr) => { write!(f, "{}", bsr) }
            BasicShape::Circle(rs, pos) => {
                write!(f, "circle({}{})", rs, pos.map(|v| format!(" at {}", v)).unwrap_or_default())
            }
            BasicShape::Ellipse(rs, pos) => {
                write!(f, "ellipse({}{})", rs, pos.map(|v| format!(" at {}", v)).unwrap_or_default())
            }
            BasicShape::Polygon(fr, rl, corners) => {
                let mut result = fr.map(|v| format!("{}{}", v, if rl.is_some() { " "} else { ""})).unwrap_or_default();
                if let Some(rl) = rl {
                    result.push_str(&rl.to_string());
                }
                if let Some(corners)= corners {
                    if result.len() > 0 {
                        result.push(',');
                    }
                    result.push_str(&corners.iter().map(|c| format!("{} {}", c.0, c.1)).collect::<Vec<_>>().join(" "));
                }
                write!(f, "polygon({})", result)
            }
            BasicShape::Path(fr, s) => {
                write!(f, "path({}{})", fr.map(|v| format!("{} ", v)).unwrap_or_default(), s)
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum VisualBox {
    ContentBox,
    PaddingBox,
    BorderBox
}

impl Display for VisualBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            VisualBox::ContentBox => { "content-box" }
            VisualBox::PaddingBox => { "padding-box" }
            VisualBox::BorderBox => { "border-box" }
        })
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ShapeBox {
    VisualBox(VisualBox),
    MarginBox
}

impl Display for ShapeBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeBox::VisualBox(vb) => { write!(f, "{}", vb)}
            ShapeBox::MarginBox => { write!(f, "margin-box") }
        }

    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum GeometryBox {
    ShapeBox(ShapeBox),
    FillBox,
    StrokeBox,
    ViewBox,
}

impl Display for GeometryBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                GeometryBox::ShapeBox(sb) => { sb.to_string() }
                GeometryBox::FillBox => { "fill-box".to_string() }
                GeometryBox::StrokeBox => { "stroke-box".to_string() }
                GeometryBox::ViewBox => { "view-box".to_string() }
            }
            )
    }
}
