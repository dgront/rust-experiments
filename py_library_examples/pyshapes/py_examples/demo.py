from shapes import Shape, Vertex

s = Shape()
s.add_vertex( Vertex(1.0, 2.0))
s.add_vertex( Vertex(3.0, 4.0))
s.add_vertex( Vertex(5.0, 6.0))


# iteration also works
for v in s.get_vertices():
    print(v)

cm = s.center()

print(s.is_point_in_polygon(cm))

