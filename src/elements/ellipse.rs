use crate::SvgElement;
use std::collections::HashMap;

pub struct Ellipse {
    center: (f32, f32),
    radius: (f32, f32),
    attributes: HashMap<String, String>,
    value: Option<String>,
}

impl Ellipse {
    pub fn center_pos(&self) -> (f32, f32) {
        self.center
    }
}

impl SvgElement for Ellipse {
    fn children(&self) -> Vec<Box<dyn SvgElement>> {
        vec![]
    }

    fn attributes(&self) -> HashMap<String, String> {
        self.attributes.clone()
    }

    fn attributes_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.attributes
    }

    fn name(&self) -> String {
        "ellipse".to_string()
    }
}
