trait MyParamTrait { fn compute_param(&self) -> f64; }

struct Param1;
impl MyParamTrait for Param1 { fn compute_param(&self) -> f64 { 2.0 } }
struct Param2;
impl MyParamTrait for Param2 { fn compute_param(&self) -> f64 { 4.0 } }

trait MyEvalTrait { fn eval(&self, x: &dyn MyParamTrait) -> f64; }


struct Eval1;

impl MyEvalTrait for Eval1 {

    fn eval(&self, x: &dyn MyParamTrait) -> f64 { x.compute_param() * 2.0 }
}

pub fn main(){

    let p1 = Param1{};
    let _p2 = Param2{};
    let e = Eval1{};
    println!("{}", e.eval(&p1));
    // so far, so good ... but I need to store MyObjects
    let mut all_my_obj: Vec<Box<dyn MyEvalTrait>> = Vec::new();
    all_my_obj.push(Box::new(e));
}


// trait MyParamTrait { fn compute_param(&self) -> f64; }
//
// struct Param1;
// impl MyParamTrait for Param1 { fn compute_param(&self) -> f64 { 2.0 } }
// struct Param2;
// impl MyParamTrait for Param2 { fn compute_param(&self) -> f64 { 4.0 } }
//
// trait MyEvalTrait<P> { fn eval(&self, x: &P) -> f64; }
//
// struct Eval1;
// impl MyEvalTrait<Param1> for Eval1 { fn eval(&self, x: &Param1) -> f64 { x.compute_param() * 2.0 } }
//
// pub fn main(){
//
//     let p1 = Param1{};
//     let _p2 = Param2{};
//     let e = Eval1{};
//     println!("{}", e.eval(&p1));
//     // so far, so good ... but I need to store MyObjects
//     let mut all_my_obj: Vec<Box<dyn MyEvalTrait<Param1>>> = Vec::new();
//     all_my_obj.push(Box::new(e));
// }