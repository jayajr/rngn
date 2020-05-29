use ::rngn::linalg::matrix::Matrix;

fn main() {
    println!("Hello, world!");
    let a = Matrix::default(2, 5);
    println!("{:?}", a);

    let mut b = a.clone();
    b.data[0][0] = 2.0;
    println!("{:?}", b);

    let c = a-b;
    println!("{:?}", c.unwrap());
}
