from pyshapes import Shape, Vertex

class C:
    def __init__(self) -> None:
        self.__x = 0

    @property
    def x(self):
        return self.__x

    @x.setter
    def set_x(self, value):
        self.__x = value

c1 = C()
print(c1.x)
c1.x = 10

s = Shape()
v = Vertex(0,0)
s.add_vertex(v)
color_copy = s.color
s.color.lighter(10)
print(s.color)

