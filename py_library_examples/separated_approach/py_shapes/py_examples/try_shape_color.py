from pyshapes import Shape, Vertex

s = Shape()
v = Vertex(0,0)
s.add_vertex(v)
s.color.lighter(10)
print(s.color)

