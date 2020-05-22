mod LinAlgEngine;

fn main() {
    println!("Hello, world!");
    let a = LinAlgEngine::Vector::new(3.2, 3., 6.);
    println!("{:?}", a)
}
