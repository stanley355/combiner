// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let mat = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("Matrix: ");
    println!("({:?} {:?})", mat.0, mat.1);
    println!("({:?} {:?})", mat.2, mat.3);
}
