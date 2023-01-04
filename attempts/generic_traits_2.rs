use std::fmt;

trait Foo<T1: fmt::Display, T2: fmt::Display> { fn foo(&self, t: &T1); }
trait Bar<T1: fmt::Display, T2: fmt::Display> { fn bar(&self, t: &T2); }

trait FooBar<T1: fmt::Display, T2: fmt::Display> : Foo<T1, T2> + Bar<T1, T2> {}

struct Point<T1: fmt::Display, T2: fmt::Display> {
    x: T1,
    y: T2
}

impl<T1: fmt::Display, T2: fmt::Display>  Foo<T1, T2> for Point<T1, T2> {

    fn foo(&self, t: &T1) { println!("foo says {} times: {},{}", t, self.x, self.y); }
}

impl<T1: fmt::Display, T2: fmt::Display>  Bar<T1, T2> for Point<T1, T2> {

    fn bar(&self, t: &T2) { println!("bar says {} times: {},{}", t, self.x, self.y); }
}

impl<T1: fmt::Display, T2: fmt::Display> FooBar<T1, T2> for Point<T1, T2> {}

fn try_foo_bar<E: FooBar<i16,i16>>(p: &E) {
    p.foo(&0);
    p.bar(&1);
}

pub fn main(){

    let p = Point{x:5i16, y:4i16};
    try_foo_bar(&p);
}