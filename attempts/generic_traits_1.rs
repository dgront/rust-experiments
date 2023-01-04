use std::fmt;

trait MyTrait {}
impl MyTrait for i16{}

trait Foo<T: fmt::Display> { fn foo(&self, t: &T); }
trait Bar<T: fmt::Display> { fn bar(&self, t: &T); }

trait FooBar<T: fmt::Display> : Foo<T> + Bar<T> {}

struct Point<T: fmt::Display> {
    x: T,
    y: T
}

impl<T: fmt::Display>  Foo<T> for Point<T> {

    fn foo(&self, t: &T) { println!("foo says {} times: {},{}", t, self.x, self.y); }
}

impl<T: fmt::Display>  Bar<T> for Point<T> {

    fn bar(&self, t: &T) { println!("bar says {} times: {},{}", t, self.x, self.y); }
}

impl<T: fmt::Display> FooBar<T> for Point<T> {}

fn try_foo_bar<E: FooBar<i16>>(p: &E) {
    p.foo(&0);
    p.bar(&1);
}

pub fn main(){

    let p = Point{x:5i16, y:4i16};
    try_foo_bar(&p);
}