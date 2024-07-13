from pyshapes import Shape, Vertex

s = Shape()
p = Vertex(1, 1)
for x, y in [(0,0), (0, 2), (2, 2), (2, 0)]:
    s.add_vertex(Vertex(x, y))
outcome = s.is_point_in_inside(p)
print(outcome)

