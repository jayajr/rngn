use ::rngn::LinAlgEngine::Vector;

fn main() {
    println!("Hello, world!");
    let a = Vector::new(3.0 as f32, 3, std::f64::consts::PI);
    println!("{:?}", a)
}
