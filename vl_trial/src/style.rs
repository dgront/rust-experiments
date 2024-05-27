use std::fmt;

pub fn rgb_to_hex(r: u16, g: u16, b: u16) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

#[derive(Debug, Clone)]
pub struct Style {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f32>,
    pub opacity: Option<f32>,
    pub fill_opacity: Option<f32>,
    pub stroke_opacity: Option<f32>,
    pub angle: f32,
}

impl Style {
    pub fn new() -> Self {
        Style {
            fill: None,
            stroke: None,
            stroke_width: None,
            opacity: None,
            fill_opacity: None,
            stroke_opacity: None,
            angle: 0.0,
        }
    }

    pub fn set_fill(&mut self, fill: &str) {
        self.fill = Some(fill.to_string());
    }

    pub fn set_stroke(&mut self, stroke: &str) {
        self.stroke = Some(stroke.to_string());
    }

    pub fn set_stroke_width(&mut self, stroke_width: f32) {
        self.stroke_width = Some(stroke_width);
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = Some(opacity);
    }

    pub fn set_fill_opacity(&mut self, fill_opacity: f32) {
        self.fill_opacity = Some(fill_opacity);
    }

    pub fn set_stroke_opacity(&mut self, stroke_opacity: f32) {
        self.stroke_opacity = Some(stroke_opacity);
    }

    pub fn set_angle(&mut self, angle_deg: f32) {
        self.angle = angle_deg;
    }

    pub fn to_string(&self) -> String {
        let mut style_string = String::new();

        if let Some(ref fill) = self.fill {
            style_string.push_str(&format!("fill:{};", fill));
        }

        if let Some(ref stroke) = self.stroke {
            style_string.push_str(&format!("stroke:{};", stroke));
        }

        if let Some(stroke_width) = self.stroke_width {
            style_string.push_str(&format!("stroke-width:{};", stroke_width));
        }

        if let Some(opacity) = self.opacity {
            style_string.push_str(&format!("opacity:{};", opacity));
        }

        if let Some(fill_opacity) = self.fill_opacity {
            style_string.push_str(&format!("fill-opacity:{};", fill_opacity));
        }

        if let Some(stroke_opacity) = self.stroke_opacity {
            style_string.push_str(&format!("stroke-opacity:{};", stroke_opacity));
        }

        style_string
    }
}
