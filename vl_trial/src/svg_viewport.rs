use crate::ToSvg;

pub struct SvgDrawing {
    width: f32,
    height: f32,
    elements: Vec<Box<dyn ToSvg>>
}

impl SvgDrawing {
    pub fn new(width: f32, height: f32) -> Self {
        SvgDrawing { width, height, elements: vec![] }
    }

    pub fn draw(&mut self) {
        println!("{}", self.svg_header());
        for element in &self.elements {
            println!("{}", element.to_svg());
        }
        println!("</svg>");
    }

    pub fn add_element(&mut self, el: Box<dyn ToSvg>) {
        self.elements.push(el);
    }

    pub fn svg_header(&self) -> String {
        let w = self.width;
        format!(r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#, self.width, self.height).to_string()
    }
}
