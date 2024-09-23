use crate::{Attribute, SvgElement, UkkoError, UkkoResult};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum PathCommandKind {
    MoveTo,
    LineTo,
    HorizontalLineTo,
    VerticalLineTo,
    CubicBezierCurve((f32, f32), (f32, f32)),
    CubicBezierCurveSmooth((f32, f32)),
    QuadraticBezierCurve((f32, f32)),
    QuadraticBezierCurveSmooth,
    EllipticalArcCurve(f32, f32, f32, bool, bool),
    ClosePath,
}

impl PathCommandKind {
    pub fn as_char(&self) -> char {
        match self {
            PathCommandKind::MoveTo => 'M',
            PathCommandKind::LineTo => 'L',
            PathCommandKind::HorizontalLineTo => 'H',
            PathCommandKind::VerticalLineTo => 'V',
            PathCommandKind::CubicBezierCurve(_, _) => 'C',
            PathCommandKind::CubicBezierCurveSmooth(_) => 'S',
            PathCommandKind::QuadraticBezierCurve(_) => 'Q',
            PathCommandKind::QuadraticBezierCurveSmooth => 'T',
            PathCommandKind::EllipticalArcCurve(_, _, _, _, _) => 'A',
            PathCommandKind::ClosePath => 'Z',
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct PathCommand {
    pub relative: bool,
    pub movement: (f32, f32),
    pub command: PathCommandKind,
}

impl PathCommand {
    pub fn relative(mut self) -> Self {
        self.relative = true;
        self
    }

    pub fn move_to(movement: (f32, f32)) -> Self {
        Self {
            relative: false,
            movement,
            command: PathCommandKind::MoveTo,
        }
    }

    pub fn line_to(movement: (f32, f32)) -> Self {
        Self {
            relative: false,
            movement,
            command: PathCommandKind::LineTo,
        }
    }

    pub fn horizontal_line_to(to: f32) -> Self {
        Self {
            relative: false,
            movement: (to, 0.),
            command: PathCommandKind::HorizontalLineTo,
        }
    }

    pub fn vertical_line_to(to: f32) -> Self {
        Self {
            relative: false,
            movement: (0., to),
            command: PathCommandKind::VerticalLineTo,
        }
    }

    pub fn cubic_bezier_curve(
        movement: (f32, f32),
        control_start: (f32, f32),
        control_end: (f32, f32),
    ) -> Self {
        Self {
            relative: false,
            movement,
            command: PathCommandKind::CubicBezierCurve(control_start, control_end),
        }
    }

    pub fn cubic_bezier_curve_smooth(movement: (f32, f32), control: (f32, f32)) -> Self {
        Self {
            relative: false,
            movement,
            command: PathCommandKind::CubicBezierCurveSmooth(control),
        }
    }

    pub fn quadratic_bezier_curve(movement: (f32, f32), control: (f32, f32)) -> Self {
        Self {
            relative: false,
            movement,
            command: PathCommandKind::QuadraticBezierCurve(control),
        }
    }

    pub fn quadratic_bezier_curve_smooth(movement: (f32, f32)) -> Self {
        Self {
            relative: false,
            movement,
            command: PathCommandKind::QuadraticBezierCurveSmooth,
        }
    }

    pub fn elliptical_arc_curve(
        movement: (f32, f32),
        rx: f32,
        ry: f32,
        angle: f32,
        large_arc_flag: bool,
        sweep_flag: bool,
    ) -> Self {
        Self {
            relative: false,
            movement,
            command: PathCommandKind::EllipticalArcCurve(rx, ry, angle, large_arc_flag, sweep_flag),
        }
    }

    pub fn close() -> Self {
        Self {
            relative: false,
            movement: (0., 0.),
            command: PathCommandKind::ClosePath,
        }
    }

    fn tuple_from_str(str: &str) -> UkkoResult<(f32, f32)> {
        let splits = str.trim().split(',').collect::<Vec<_>>();
        Ok((
            splits
                .get(0)
                .ok_or(UkkoError::parse("Not a tuple."))?
                .parse::<f32>()?,
            splits
                .get(1)
                .ok_or(UkkoError::parse("Not a tuple."))?
                .parse::<f32>()?,
        ))
    }

    pub fn from_str(str: &str) -> Result<Self, UkkoError> {
        let str = str.trim();
        let splits = str.split_whitespace().collect::<Vec<_>>();
        if splits.is_empty() || splits[0].len() > 1 {
            return Err(UkkoError::parse("Not a command."));
        }
        let char = splits[0].chars().next().unwrap();
        let command = match char.to_ascii_uppercase() {
            'M' => Self::move_to(Self::tuple_from_str(
                splits
                    .get(1)
                    .ok_or(UkkoError::parse("Not enough arguments."))?,
            )?),
            'L' => Self::line_to(Self::tuple_from_str(
                splits
                    .get(1)
                    .ok_or(UkkoError::parse("Not enough arguments."))?,
            )?),
            'H' => Self::horizontal_line_to(
                splits
                    .get(1)
                    .ok_or(UkkoError::parse("Not enough arguments."))?
                    .parse::<f32>()?,
            ),
            'V' => Self::vertical_line_to(
                splits
                    .get(1)
                    .ok_or(UkkoError::parse("Not enough arguments."))?
                    .parse::<f32>()?,
            ),
            'C' => Self::cubic_bezier_curve(
                Self::tuple_from_str(
                    splits
                        .get(3)
                        .ok_or(UkkoError::parse("Not enough arguments."))?,
                )?,
                Self::tuple_from_str(
                    splits
                        .get(1)
                        .ok_or(UkkoError::parse("Not enough arguments."))?,
                )?,
                Self::tuple_from_str(
                    splits
                        .get(2)
                        .ok_or(UkkoError::parse("Not enough arguments."))?,
                )?,
            ),
            'S' => Self::cubic_bezier_curve_smooth(
                Self::tuple_from_str(
                    splits
                        .get(2)
                        .ok_or(UkkoError::parse("Not enough arguments."))?,
                )?,
                Self::tuple_from_str(
                    splits
                        .get(1)
                        .ok_or(UkkoError::parse("Not enough arguments."))?,
                )?,
            ),
            'Q' => Self::quadratic_bezier_curve(
                Self::tuple_from_str(
                    splits
                        .get(2)
                        .ok_or(UkkoError::parse("Not enough arguments."))?,
                )?,
                Self::tuple_from_str(
                    splits
                        .get(1)
                        .ok_or(UkkoError::parse("Not enough arguments."))?,
                )?,
            ),
            'T' => Self::quadratic_bezier_curve_smooth(Self::tuple_from_str(
                splits
                    .get(1)
                    .ok_or(UkkoError::parse("Not enough arguments."))?,
            )?),
            'A' => Self::elliptical_arc_curve(
                Self::tuple_from_str(
                    splits
                        .get(6)
                        .ok_or(UkkoError::parse("Not enough arguments."))?,
                )?,
                splits
                    .get(1)
                    .ok_or(UkkoError::parse("Not enough arguments."))?
                    .parse::<f32>()?,
                splits
                    .get(2)
                    .ok_or(UkkoError::parse("Not enough arguments."))?
                    .parse::<f32>()?,
                splits
                    .get(3)
                    .ok_or(UkkoError::parse("Not enough arguments."))?
                    .parse::<f32>()?,
                splits
                    .get(4)
                    .ok_or(UkkoError::parse("Not enough arguments."))?
                    .parse::<i8>()?
                    != 0,
                splits
                    .get(5)
                    .ok_or(UkkoError::parse("Not enough arguments."))?
                    .parse::<i8>()?
                    != 0,
            ),
            'Z' => PathCommand::close(),
            _ => {
                return Err(UkkoError::parse("Not a command."));
            }
        };

        let command = if char.is_lowercase() {
            command.relative()
        } else {
            command
        };

        Ok(command)
    }

    fn fmt_movement(&self) -> String {
        Self::fmt_tuple(self.movement)
    }

    fn fmt_tuple(tup: (f32, f32)) -> String {
        format!("{},{}", tup.0, tup.1)
    }
}

impl Display for PathCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = self.command.as_char();
        let char = if self.relative {
            char.to_ascii_lowercase()
        } else {
            char
        };
        let str = match self.command {
            PathCommandKind::MoveTo
            | PathCommandKind::LineTo
            | PathCommandKind::QuadraticBezierCurveSmooth => self.fmt_movement(),
            PathCommandKind::HorizontalLineTo => self.movement.0.to_string(),
            PathCommandKind::VerticalLineTo => self.movement.1.to_string(),
            PathCommandKind::CubicBezierCurve(c1, c2) => {
                format!(
                    "{} {} {}",
                    Self::fmt_tuple(c1),
                    Self::fmt_tuple(c2),
                    self.fmt_movement()
                )
            }
            PathCommandKind::CubicBezierCurveSmooth(c) => {
                format!("{} {}", Self::fmt_tuple(c), self.fmt_movement())
            }
            PathCommandKind::QuadraticBezierCurve(q) => {
                format!("{} {}", Self::fmt_tuple(q), self.fmt_movement())
            }
            PathCommandKind::EllipticalArcCurve(rx, ry, angle, large_arc_flag, sweep_flag) => {
                format!(
                    "{} {} {} {} {} {}",
                    rx,
                    ry,
                    angle,
                    large_arc_flag as i8,
                    sweep_flag as i8,
                    self.fmt_movement()
                )
            }
            PathCommandKind::ClosePath => {
                return write!(f, "{}", char);
            }
        };

        write!(f, "{} {}", char, str)
    }
}

#[derive(PartialEq, Debug)]
pub struct PathShape {
    pub elements: Vec<PathCommand>,
}

impl PathShape {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    pub fn to_path(self) -> Path {
        Path {
            shape: self,
            attributes: Default::default(),
            value: None,
        }
    }

    pub fn with_commands(mut self, commands: Vec<PathCommand>) -> Self {
        self.elements = commands;
        self
    }

    pub fn from_str(str: &str) -> UkkoResult<Self> {
        let chars = &str.matches(|c: char| c.is_alphabetic()).collect::<Vec<_>>();
        let splits = &str.split(|c: char| c.is_alphabetic()).collect::<Vec<_>>()[1..];
        let splits = splits
            .into_iter()
            .zip(chars)
            .map(|(a, b)| format!("{} {}", b.trim(), a.trim()).trim().to_string())
            .collect::<Vec<_>>();
        println!("{:#?}", splits);
        Ok(Self {
            elements: splits
                .into_iter()
                .map(|s| PathCommand::from_str(&s))
                .collect::<UkkoResult<_>>()?,
        })
    }
}

impl Attribute for PathShape {
    fn key(&self) -> String {
        "d".to_string()
    }

    fn value(&self) -> String {
        self.to_string()
    }

    fn from_key_value(kv: (String, String)) -> UkkoResult<Self> {
        if kv.0.to_ascii_lowercase() != "d" { return Err(UkkoError::TODO); }
        Self::from_str(kv.1.as_str())
    }
}

impl Display for PathShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let elements = self
            .elements
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", elements)
    }
}

pub struct Path {
    shape: PathShape,
    attributes: HashMap<String, String>,
    value: Option<String>,
}

impl SvgElement for Path {
    fn children(&self) -> Vec<Box<dyn SvgElement>> {
        vec![]
    }

    fn attributes(&self) -> HashMap<String, String> {
        let mut map = self.attributes.clone();
        map.insert("d".to_string(), self.shape.to_string());
        map
    }

    fn attributes_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.attributes
    }

    fn name(&self) -> String {
        "path".to_string()
    }

    fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;

    #[test]
    fn test_to_xml() {
        let shape = PathShape::from_str("M 10,10 Z").unwrap().to_path();
        let node = shape.to_xml_node();
        let mut buf = BufWriter::new(Vec::new());
        node.as_element().unwrap().write(&mut buf).unwrap();
        assert_eq!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><path d=\"M 10,10&#xA;Z\" />",
            String::from_utf8(buf.into_inner().unwrap())
                .unwrap()
                .as_str()
        );
    }

    #[test]
    fn test_shape_parse() {
        let c_str = "M 10,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(command, PathCommand::move_to((10., 10.)));
        let c_str = "L 10,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(command, PathCommand::line_to((10., 10.)));
        let c_str = "H 10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(command, PathCommand::horizontal_line_to(10.));
        let c_str = "V 10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(command, PathCommand::vertical_line_to(10.));
        let c_str = "C 10,10 10,10 3.5,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(
            command,
            PathCommand::cubic_bezier_curve((3.5, 10.), (10., 10.), (10., 10.))
        );
        let c_str = "S 10,10 3.5,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(
            command,
            PathCommand::cubic_bezier_curve_smooth((3.5, 10.), (10., 10.))
        );
        let c_str = "Q 10,10 3.5,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(
            command,
            PathCommand::quadratic_bezier_curve((3.5, 10.), (10., 10.))
        );
        let c_str = "T 3.5,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(
            command,
            PathCommand::quadratic_bezier_curve_smooth((3.5, 10.))
        );
        let c_str = "A 1 1 1 0 1 3.5,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(
            command,
            PathCommand::elliptical_arc_curve((3.5, 10.), 1., 1., 1., false, true)
        );
        let c_str = "Z";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(command, PathCommand::close());

        let c_str = "m 10,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(command, PathCommand::move_to((10., 10.)).relative());
        let c_str = "l 10,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(command, PathCommand::line_to((10., 10.)).relative());
        let c_str = "h 10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(command, PathCommand::horizontal_line_to(10.).relative());
        let c_str = "v 10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(command, PathCommand::vertical_line_to(10.).relative());
        let c_str = "c 10,10 10,10 3.5,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(
            command,
            PathCommand::cubic_bezier_curve((3.5, 10.), (10., 10.), (10., 10.)).relative()
        );
        let c_str = "s 10,10 3.5,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(
            command,
            PathCommand::cubic_bezier_curve_smooth((3.5, 10.), (10., 10.)).relative()
        );
        let c_str = "q 10,10 3.5,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(
            command,
            PathCommand::quadratic_bezier_curve((3.5, 10.), (10., 10.)).relative()
        );
        let c_str = "t 3.5,10";
        let command = PathCommand::from_str(c_str).unwrap();
        assert_eq!(
            command,
            PathCommand::quadratic_bezier_curve_smooth((3.5, 10.)).relative()
        );
        let c_str = "a 1 1 1 0 1 3.5,10";
        let command1 = PathCommand::from_str(c_str).unwrap();
        assert_eq!(
            command1,
            PathCommand::elliptical_arc_curve((3.5, 10.), 1., 1., 1., false, true).relative()
        );
        let c_str = "z";
        let command2 = PathCommand::from_str(c_str).unwrap();
        assert_eq!(command2, PathCommand::close().relative());
        let c_str = "a 1 1 1 0 1 3.5,10. z";
        let shape = PathShape::from_str(c_str).unwrap();
        assert_eq!(
            shape,
            PathShape::new().with_commands(vec![command1, command2])
        )
    }

    #[test]
    fn test_shape_fmt() {
        let command1 = PathCommand::line_to((10., 10.));
        let command2 = PathCommand::move_to((20., 20.));
        let command3 = PathCommand::horizontal_line_to(3.5);
        let command4 = PathCommand::vertical_line_to(3.5);
        let command5 = PathCommand::cubic_bezier_curve((10., 10.), (5., 5.), (4., 4.));
        let command6 = PathCommand::cubic_bezier_curve_smooth((10., 10.), (5., 5.));
        let command7 = PathCommand::quadratic_bezier_curve((10., 10.), (5., 5.));
        let command8 = PathCommand::quadratic_bezier_curve_smooth((10., 10.));
        let command9 = PathCommand::elliptical_arc_curve((10., 10.), 4., 1., 0.3, false, true);
        let command10 = PathCommand::close();

        let shape = PathShape {
            elements: vec![
                command1, command2, command3, command4, command5, command6, command7, command8,
                command9, command10,
            ],
        };
        assert_eq!(
            "\
        L 10,10\n\
        M 20,20\n\
        H 3.5\n\
        V 3.5\n\
        C 5,5 4,4 10,10\n\
        S 5,5 10,10\n\
        Q 5,5 10,10\n\
        T 10,10\n\
        A 4 1 0.3 0 1 10,10\n\
        Z",
            shape.to_string().as_str()
        )
    }
}
