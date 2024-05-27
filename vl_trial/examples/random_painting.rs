use std::env;
use rand::Rng;
use visualife::colors::darker;
use visualife::shapes::{Circle, Rect};
use visualife::style::{rgb_to_hex, Style};
use visualife::ToSvg;
use visualife::SvgDrawing;

fn main() {
    let draw_width = 1000.0;
    let (n_x, n_y) = (100, 100);
    let box_width = 9.0;
    let mut rng = rand::thread_rng();
    let mut shapes: Vec<Box<dyn ToSvg>> = Vec::new();
    let mut drawing = SvgDrawing::new(draw_width, draw_width);
    let max_noise: f32 = 0.5;
    for i in 0..n_x {
        let mut max_drop = i as f32 / 100.0 + 1.0;
        max_drop = max_drop * max_drop * max_drop;
        for j in 0..n_y {
            let mut style = Style::new();
            let fill = rgb_to_hex(255, i * 255 / n_x, j * 255 / n_y);
            style.set_stroke(&darker(&fill, 0.3).ok().unwrap());
            style.set_fill(&fill);
            style.set_opacity(rng.gen_range(0.6..=1.0));
            style.set_stroke_width(rng.gen_range(0.5..=2.0));
            let noise_x = rng.gen_range(-max_noise..max_noise);
            let noise_y = rng.gen_range(-max_noise..max_noise) + rng.gen_range(max_drop/2.0..max_drop);
            let x = i as f32 * draw_width / (n_x as f32) + noise_x;
            let y = j as f32 * draw_width / (n_y as f32) + noise_y;
            let id = format!("el_{i}_{j}");
            if rng.gen_range(0.0..1.0) < 0.1 {
                let mut circ = Circle::new(&id, x, y, box_width);
                circ.style = style;
                drawing.add_element(Box::new(circ));
                // shapes.push(Box::new(circ));
            } else {
                let mut rect = Rect::new(&id, x, y, box_width, box_width);
                rect.style = style;
                let angle = rng.gen_range(0.0..=i as f32 * j as f32 * 60.0 / (n_x * n_y) as f32);
                rect.style.set_angle(angle);
                drawing.add_element(Box::new(rect));
                // shapes.push(Box::new(rect));
            }
        }
    }
    drawing.draw();
    // println!(r##"<svg width="#{draw_width}" height="{draw_width}" xmlns="http://www.w3.org/2000/svg">"##);
    // for r in shapes {
    //     println!("{}", &r.to_svg());
    // }
    // println!("</svg>");
}
