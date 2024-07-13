use shapes::{Shape, Vertex};

fn main() {
    // Create a new shape
    let mut shape = Shape::new();

    // Add vertices to the shape
    shape.add_vertex(Vertex::new(0.0, 1.0));
    shape.add_vertex(Vertex::new(0.0, 2.0));
    shape.add_vertex(Vertex::new(1.0, 1.0));

    // Print the vertices
    println!("{}", &shape);
    // Print the center
    let center = shape.center();
    println!("Center: {}", center);
    // Is the center inside?
    println!("Is the center inside? {}", shape.is_point_in_inside(&center));
}
