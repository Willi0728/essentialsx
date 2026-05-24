use essentialsx::math::Matrix;
fn main() {
    let matrix = Matrix::<10, 10>::identity();
    println!("{:?}", matrix.determinant());
}