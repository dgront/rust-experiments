use shapes::{Shape, Vertex};

fn main() {
    // Create a new shape
    let mut shape = Shape::new();

    // Add vertices to the shape
    shape.add_vertex(Vertex::new(1.0, 2.0));
    shape.add_vertex(Vertex::new(3.0, 4.0));
    shape.add_vertex(Vertex::new(5.0, 6.0));

    // Print the vertices
    println!("{}", &shape);
}
