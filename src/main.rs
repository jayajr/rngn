use ::rngn::linalg::matrix::Matrix;

fn main() {
    println!("Hello, world!");
    let a = Matrix::default(2, 5);
    println!("{:?}", a);

    let mut b = a;
    b.data[0][0] = 2.0;
    let k = b.m;
    println!("{:?}", b);
}
