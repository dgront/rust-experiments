use crate::draw_svg::ToSvg;
use crate::style::Style;

#[derive(Debug, Clone)]
pub struct Circle {
    pub id: String,
    pub cx: f32,
    pub cy: f32,
    pub radius: f32,
    pub style: Style,
}

impl Circle {
    pub fn new(id: &str, cx: f32, cy: f32, radius: f32) -> Self {
        Circle { id: id.to_string(), cx, cy, radius, style: Style::new(), }
    }
}

impl ToSvg for Circle {
    fn to_svg(&self) -> String {
        let mut svg_string = format!(
            r#"<circle id="{}" cx="{}" cy="{}" r="{}""#,
            self.id, self.cx, self.cy, self.radius
        );

        let style_string = self.style.to_string();
        if !style_string.is_empty() {
            svg_string.push_str(&format!(r#" style="{}""#, style_string));
        }

        svg_string.push_str(r#" />"#);

        svg_string
    }
}