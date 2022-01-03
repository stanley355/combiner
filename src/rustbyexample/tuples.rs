#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
  let pair = (1, true);
  println!("pair is {:?}", pair);

  println!("the reversed pair is {:?}", reverse(pair));

  // To create one element tuples, the comma is required to tell them apart
  // from a literal surrounded by parentheses
  println!("one element tuple: {:?}", (5u32,));
  println!("just an integer: {:?}", (5u32));

  //tuples can be destructured to create bindings
  let tuple = (1, "hello", 4.5, true);

  let (a, b, c, d) = tuple;
  println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{:?}", matrix.1);

  let mat = Matrix(1.1, 1.2, 2.1, 2.2);

  println!("Matrix: ");
  println!("({:?} {:?})", mat.0, mat.1);
  println!("({:?} {:?})", mat.2, mat.3);

  transpose(mat);
}

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
  // `let` can be used to bind the members of a tuple to variables
  let (integer, boolean) = pair;

  (boolean, integer)
}

fn transpose(set: Matrix) {
  // `let` can be used to bind the members of a tuple to variables
  println!("Transpose: ");
  println!("({:?} {:?})", set.0, set.2);
  println!("({:?} {:?})", set.1, set.3);
}
