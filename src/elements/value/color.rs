use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum HexColor {
    Three(u8, u8, u8),
    Four(u8, u8, u8, u8),
    Six(u8, u8, u8),
    Eight(u8, u8, u8, u8),
}

impl Display for HexColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}",
               match *self {
                   HexColor::Three(r, g, b) => {
                       format!("{:X}{:X}{:X}", r.min(15u8), g.min(15u8), b.min(15u8))
                   }
                   HexColor::Four(r, g, b, a) => {
                       format!("{:X}{:X}{:X}{:X}", r.min(15u8), g.min(15u8), b.min(15u8), a.min(15u8))
                   }
                   HexColor::Six(r, g, b) => {
                       format!("{:02X}{:02X}{:02X}",r,g,b)
                   }
                   HexColor::Eight(r,g,b,a) => {
                       format!("{:02X}{:02X}{:02X}{:02X}",r,g,b,a)
                   }
               })
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum RectangularColorSpace {
    Srgb,
    SrgbLinear,
    DisplayP3,
    A98Rgb,
    ProphotoRgb,
    Rec2020 ,
    Lab,
    OkLab,
    Xyz,
    XyzD50,
    XyzD65,
}

impl Display for RectangularColorSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RectangularColorSpace::Srgb => { "srgb"}
            RectangularColorSpace::SrgbLinear => { "srgb-linear"}
            RectangularColorSpace::DisplayP3 => { "display-p3"}
            RectangularColorSpace::A98Rgb => { "a98-rgb"}
            RectangularColorSpace::ProphotoRgb => { "prophoto-rgb"}
            RectangularColorSpace::Rec2020 => { "rec2020"}
            RectangularColorSpace::Lab => { "lab"}
            RectangularColorSpace::OkLab => { "oklab"}
            RectangularColorSpace::Xyz => { "xyz"}
            RectangularColorSpace::XyzD50 => { "xyz-d50"}
            RectangularColorSpace::XyzD65 => { "xyz-d65"}
        })
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PolarColorSpace {
    Hsl,
    Hwb,
    Lch,
    OkLch
}

impl Display for PolarColorSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PolarColorSpace::Hsl => { "hsl"}
            PolarColorSpace::Hwb => { "hwb"}
            PolarColorSpace::Lch => { "lch"}
            PolarColorSpace::OkLch => { "oklch" }
        })
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum HueInterpolationMethod {
    Shorter,
    Longer,
    Increasing,
    Decreasing
}

impl Display for HueInterpolationMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} hue", match self {
            HueInterpolationMethod::Shorter => { "shorter"}
            HueInterpolationMethod::Longer => { "longer"}
            HueInterpolationMethod::Increasing => { "increasing"}
            HueInterpolationMethod::Decreasing => { "decreasing"}
        })
    }
}


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ColorInterpolationMethod {
    RectangularColorSpace(RectangularColorSpace),
    PolarColorSpace(PolarColorSpace, Option<HueInterpolationMethod>)
}

impl Display for ColorInterpolationMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "in {}", match self {
            ColorInterpolationMethod::RectangularColorSpace(rcs) => { rcs.to_string()}
            ColorInterpolationMethod::PolarColorSpace(pcs, him) => {
                format!("{}{}", pcs, him.map(|v| format!(" {}", v)).unwrap_or_default())
            }
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CssColor {
    Keyword(String),
    Hex(HexColor),
    ColorMix(ColorInterpolationMethod, Box<CssColor>, Option<f64>, Box<CssColor>, Option<f64>),
    LightDark(Box<CssColor>, Box<CssColor>)
    // TODO: Relative & Color Function
}

impl Display for CssColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CssColor::Keyword(kw) => { write!(f, "{}", kw) }
            CssColor::Hex(hx) => { write!(f, "{}", hx) }
            CssColor::ColorMix(cip, c1, p1, c2, p2) => {
                write!(f, "color-mix({},{},{})", cip,
                       format!("{}{}",c1, p1.map(|v| format!(" {}", v)).unwrap_or_default()),
                       format!("{}{}",c2, p2.map(|v| format!(" {}", v)).unwrap_or_default()),
                )
            }
            CssColor::LightDark(l, d) => {
                write!(f, "light-dark({}, {})", l, d)

            }
        }
    }
}
