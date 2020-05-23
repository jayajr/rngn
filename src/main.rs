use ::rngn::linalg::vector::Vector;

fn main() {
    println!("Hello, world!");
    let mut a = Vector::new(3.0 as f32, 3, std::f64::consts::PI);
    a -= Vector::new(3.0, 3, std::f64::consts::PI);

    assert_eq!(a, Vector::new(0,0,0));
    println!("{:?}", a);
}
