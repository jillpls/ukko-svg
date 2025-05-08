use crate::elements::value::LengthPercentage;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionOne {
    Left,
    Center,
    Right,
    Top,
    Bottom,
    XStart,
    XEnd,
    YStart,
    YEnd,
    BlockStart,
    BlockEnd,
    InlineStart,
    InlineEnd,
    LengthPercentage(LengthPercentage),
}

impl Display for PositionOne {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let PositionOne::LengthPercentage(lp) = self {
            write!(f, "{}", lp)
        } else {
            match self {
                PositionOne::Left => "left",
                PositionOne::Center => "center",
                PositionOne::Right => "right",
                PositionOne::Top => "top",
                PositionOne::Bottom => "bottom",
                PositionOne::XStart => "x-start",
                PositionOne::XEnd => "x-end",
                PositionOne::YStart => "y-start",
                PositionOne::YEnd => "y-end",
                PositionOne::BlockStart => "block-start",
                PositionOne::BlockEnd => "block-end",
                PositionOne::InlineStart => "inline-start",
                PositionOne::InlineEnd => "inline-end",
                PositionOne::LengthPercentage(_) => {
                    unreachable!()
                }
            }
            .to_string()
            .fmt(f)
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionTwoAlignHorizontal {
    Left,
    Center,
    Right,
    XStart,
    XEnd,
    LengthPercentage(LengthPercentage),
}

impl Display for PositionTwoAlignHorizontal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionTwoAlignHorizontal::Left => {
                    "left".to_string()
                }
                PositionTwoAlignHorizontal::Center => {
                    "center".to_string()
                }
                PositionTwoAlignHorizontal::Right => {
                    "right".to_string()
                }
                PositionTwoAlignHorizontal::XStart => {
                    "x-start".to_string()
                }
                PositionTwoAlignHorizontal::XEnd => {
                    "x-end".to_string()
                }
                PositionTwoAlignHorizontal::LengthPercentage(lp) => {
                    lp.to_string()
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionTwoAlignVertical {
    Top,
    Center,
    Bottom,
    YStart,
    YEnd,
    LengthPercentage(LengthPercentage),
}

impl Display for PositionTwoAlignVertical {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionTwoAlignVertical::Top => {
                    "top".to_string()
                }
                PositionTwoAlignVertical::Center => {
                    "center".to_string()
                }
                PositionTwoAlignVertical::Bottom => {
                    "bottom".to_string()
                }
                PositionTwoAlignVertical::YStart => {
                    "y-start".to_string()
                }
                PositionTwoAlignVertical::YEnd => {
                    "y-end".to_string()
                }
                PositionTwoAlignVertical::LengthPercentage(lp) => {
                    lp.to_string()
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionTwoBlock {
    BlockStart,
    Center,
    BlockEnd,
}

impl Display for PositionTwoBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionTwoBlock::BlockStart => {
                    "block-start"
                }
                PositionTwoBlock::Center => {
                    "center"
                }
                PositionTwoBlock::BlockEnd => {
                    "block-end"
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionTwoInline {
    InlineStart,
    Center,
    InlineEnd,
}

impl Display for PositionTwoInline {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionTwoInline::InlineStart => {
                    "inline-start"
                }
                PositionTwoInline::Center => {
                    "center"
                }
                PositionTwoInline::InlineEnd => {
                    "inline-end"
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionTwoSimple {
    Start,
    Center,
    End,
}

impl Display for PositionTwoSimple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionTwoSimple::Start => {
                    "start"
                }
                PositionTwoSimple::Center => {
                    "center"
                }
                PositionTwoSimple::End => {
                    "end"
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionTwo {
    Align(PositionTwoAlignHorizontal, PositionTwoAlignVertical),
    BlockInline(PositionTwoBlock, PositionTwoInline),
    Simple(PositionTwoSimple, PositionTwoSimple),
}

impl Display for PositionTwo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (a, b) = match self {
            PositionTwo::Align(a, b) => (a.to_string(), b.to_string()),
            PositionTwo::BlockInline(a, b) => (a.to_string(), b.to_string()),
            PositionTwo::Simple(a, b) => (a.to_string(), b.to_string()),
        };
        write!(f, "{} {}", a, b)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionFourAlignHorizontal {
    Left,
    Right,
    XStart,
    XEnd,
}

impl Display for PositionFourAlignHorizontal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionFourAlignHorizontal::Left => {
                    "left"
                }
                PositionFourAlignHorizontal::Right => {
                    "right"
                }
                PositionFourAlignHorizontal::XStart => {
                    "x-start"
                }
                PositionFourAlignHorizontal::XEnd => {
                    "x-end"
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionFourAlignVertical {
    Top,
    Bottom,
    YStart,
    YEnd,
}

impl Display for PositionFourAlignVertical {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionFourAlignVertical::Top => {
                    "top"
                }
                PositionFourAlignVertical::Bottom => {
                    "bottom"
                }
                PositionFourAlignVertical::YStart => {
                    "y-start"
                }
                PositionFourAlignVertical::YEnd => {
                    "y-end"
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionFourBlock {
    BlockStart,
    BlockEnd,
}

impl Display for PositionFourBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionFourBlock::BlockStart => {
                    "block-start"
                }
                PositionFourBlock::BlockEnd => {
                    "block-end"
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionFourInline {
    InlineStart,
    InlineEnd,
}

impl Display for PositionFourInline {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionFourInline::InlineStart => {
                    "inline-start"
                }
                PositionFourInline::InlineEnd => {
                    "inline-end"
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionFourSimple {
    Start,
    End,
}

impl Display for PositionFourSimple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionFourSimple::Start => {
                    "start"
                }
                PositionFourSimple::End => {
                    "end"
                }
            }
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PositionFour {
    Align(
        (PositionFourAlignHorizontal, LengthPercentage),
        (PositionFourAlignVertical, LengthPercentage),
    ),
    BlockInline(
        (PositionFourBlock, LengthPercentage),
        (PositionFourInline, LengthPercentage),
    ),
    Simple(
        (PositionFourSimple, LengthPercentage),
        (PositionFourSimple, LengthPercentage),
    ),
}

impl Display for PositionFour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionFour::Align((a, b), (c, d)) => {
                write!(f, "{} {} {} {}", a, b, c, d)
            }
            PositionFour::BlockInline((a, b), (c, d)) => {
                write!(f, "{} {} {} {}", a, b, c, d)
            }
            PositionFour::Simple((a, b), (c, d)) => {
                write!(f, "{} {} {} {}", a, b, c, d)
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Position {
    One(PositionOne),
    Two(PositionTwo),
    Four(PositionFour),
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Position::One(po) => {
                    po.to_string()
                }
                Position::Two(pt) => {
                    pt.to_string()
                }
                Position::Four(pf) => {
                    pf.to_string()
                }
            }
        )
    }
}
