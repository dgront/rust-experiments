import sys
sys.path.append("C:\src.git\rust-experiments\py_library_examples\pyshapes\target\debug")
import shapes
print(dir(shapes))
s = shapes.Shape()
s.add_vertex( Vertex(1.0, 2.0))
s.add_vertex( Vertex(3.0, 4.0))
s.add_vertex( Vertex(5.0, 6.0))