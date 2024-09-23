use xmltree::{Element, XMLNode};
use ukko_svg::elements::path::*;
use ukko_svg::SvgElement;

fn main() {
    let str = "M 10,10 S 1,1 10,10 z";
    let path = PathShape::from_str(str).unwrap().to_path();
    let node = path.to_xml_node();
    let mut top_element = Element::new("svg");
    top_element.attributes.insert("viewBox".to_string(), "0 0 100 100".to_string());
    top_element.attributes.insert("xmlns".to_string(), "http://www.w3.org/2000/svg".to_string());
    top_element.children.push(node);
    top_element.write(std::fs::File::create("test.svg").unwrap());
}