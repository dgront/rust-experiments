use std::any::Any;

trait TypeA {
    fn as_any(&self) -> &dyn Any;
}

trait TypeB: Any {
    fn specific_to_type_b(&self);
}

struct MyTypeAB {
    pub x: usize
}


impl TypeA for MyTypeAB {
    fn as_any(&self) -> &dyn Any {
        self
    }

}

pub fn as_element<T: 'static>(el: &dyn Any) -> Option<&T> {
    el.downcast_ref::<T>()
}


impl TypeB for MyTypeAB {
    fn specific_to_type_b(&self) {
        println!("MyTypeAB::specific_to_type_b called");
    }
}

fn main() {
    let mut elements: Vec<Box<dyn TypeA>> = Vec::new();
    elements.push(Box::new(MyTypeAB{x:1}));

    for element in &elements {
        if let Some(type_b) = element.as_any().downcast_ref::<MyTypeAB>() {
            println!("Element implements TypeB");
            type_b.specific_to_type_b();
        } else {
            println!("Element does not implement TypeB");
        }
    }

    // --- any element is of type Box<dyn TypeA>
    let first_element: &mut Box<dyn TypeA>  = &mut elements[0];
    // --- but they may be casted to the original struct
    let op = as_element::<MyTypeAB>(elements[0].as_any());

    // let struct_AB: &MyTypeAB = as_element(&first_element).unwrap();
}

