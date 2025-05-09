use std::fmt::Display;
use std::fmt::Formatter;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DisplayOutside {
    Block,
    Inline,
    RunIn
}

impl Display for DisplayOutside {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use DisplayOutside::*;
        match self {
            Block => f.write_str("block"),
            Inline => f.write_str("inline"),
            RunIn => f.write_str("run-in"),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DisplayInside {
    Flow,
    FlowRoot,
    Table,
    Flex,
    Grid,
    Ruby
}

impl Display for DisplayInside {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use DisplayInside::*;
        match self {
            Flow => f.write_str("flow"),
            FlowRoot => f.write_str("flow-root"),
            Table => f.write_str("table"),
            Flex => f.write_str("flex"),
            Grid => f.write_str("grid"),
            Ruby => f.write_str("ruby"),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DisplayInsideMath {
    Inside(DisplayInside),
    Math
}

impl Display for DisplayInsideMath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inside(di) => di.fmt(f),
            Self::Math => write!(f, "math"),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DisplayOutsideInside {
    OutsideInside(DisplayOutside, DisplayInsideMath),
    Outside(DisplayOutside),
    Inside(DisplayInsideMath),
}

impl Display for DisplayOutsideInside {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use DisplayOutsideInside::*;
        match self {
            OutsideInside(o, i) => {
                write!(f, "{} {}", o, i)
            },
            Outside(o) => { o.fmt(f) }
            Inside(i) => { i.fmt(f) }
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum FlowOrRoot {
    Flow,
    FlowRoot
}

impl Display for FlowOrRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use FlowOrRoot::*;
        match self {
            Flow => f.write_str("flow"),
            FlowRoot => f.write_str("flow-root"),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DisplayListItem(Option<DisplayOutside>, Option<FlowOrRoot>);

impl Display for DisplayListItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        if let Some(d) = self.0 {
            result.push_str(&d.to_string());
            result.push(' ');
        }
        if let Some(f) = self.1 {
            result.push_str(&f.to_string());
            result.push(' ');
        }
        result.push_str("list-item");
        f.write_str(&result)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DisplayInternal {
    TableRowGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRow,
    TableCell,
    TableColumnGroup,
    TableColumn,
    TableCaption,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,
}

impl Display for DisplayInternal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use DisplayInternal::*;
        match self {
            TableRowGroup => f.write_str("table-row-group"),
            TableHeaderGroup => f.write_str("table-header-group"),
            TableFooterGroup => f.write_str("table-footer-group"),
            TableRow => f.write_str("table-row"),
            TableCell => f.write_str("table-cell"),
            TableColumnGroup => f.write_str("table-column-group"),
            TableColumn => f.write_str("table-column"),
            TableCaption => f.write_str("table-caption"),
            RubyBase => f.write_str("ruby-base"),
            RubyText => f.write_str("ruby-text"),
            RubyBaseContainer => f.write_str("ruby-base-container"),
            RubyTextContainer => f.write_str("ruby-text-container"),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DisplayBox {
    Contents,
    None
}

impl Display for DisplayBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use DisplayBox::*;
        match self {
            Contents => f.write_str("contents"),
            None => f.write_str("none"),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DisplayLegacy {
    InlineBlock,
    InlineTable,
    InlineFlex,
    InlineGrid
}

impl Display for DisplayLegacy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use DisplayLegacy::*;
        match self {
            InlineBlock => f.write_str("inline-block"),
            InlineTable => f.write_str("inline-table"),
            InlineFlex => f.write_str("inline-flex"),
            InlineGrid => f.write_str("inline-grid"),
        }
    }
}
