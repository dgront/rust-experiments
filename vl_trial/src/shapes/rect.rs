use std::fmt;
use crate::draw_svg::ToSvg;
use crate::style::Style;


#[derive(Debug, Clone)]
pub struct Rect {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rx: Option<f32>,
    pub ry: Option<f32>,
    pub style: Style,
}

impl Rect {
    pub fn new(id: &str, x: f32, y: f32, width: f32, height: f32) -> Self {
        Rect { id: id.to_string(), x, y, width, height, rx: None, ry: None, style: Style::new(), }
    }

    pub fn set_rx(&mut self, rx: f32) { self.rx = Some(rx); }

    pub fn set_ry(&mut self, ry: f32) { self.ry = Some(ry); }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_svg())
    }
}

impl ToSvg for Rect {
    fn to_svg(&self) -> String {
        let mut svg_string = format!(
            r#"<rect id="{}" x="{}" y="{}" width="{}" height="{}""#,
            self.id, self.x, self.y, self.width, self.height
        );

        if let Some(rx) = self.rx {
            svg_string.push_str(&format!(r#" rx="{}""#, rx));
        }

        if let Some(ry) = self.ry {
            svg_string.push_str(&format!(r#" ry="{}""#, ry));
        }

        let style_string = self.style.to_string();
        if !style_string.is_empty() {
            svg_string.push_str(&format!(r#" style="{}""#, style_string));
        }

        if self.style.angle != 0.0 {
            svg_string.push_str(&format!(
                r#" transform="rotate({} {} {})""#,
                self.style.angle, self.x + self.width / 2.0, self.y + self.height / 2.0
            ));
        }

        svg_string.push_str(r#" />"#);

        svg_string
    }
}

