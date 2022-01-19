// `test!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
macro_rules! test {
  // Arguments don't need to be separated by a comma.
  // Any template can be used!
  ($left:expr; and $right:expr) => {
      println!("{:?} and {:?} is {:?}",
               stringify!($left),
               stringify!($right),
               $left && $right)
  };
  // ^ each arm must end with a semicolon.
  ($left:expr; or $right:expr) => {
      println!("{:?} or {:?} is {:?}",
               stringify!($left),
               stringify!($right),
               $left || $right)
  };
}

fn main() {
  test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
  test!(true; or false);
}

// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
  // Base case:
  ($x:expr) => ($x);
  // `$x` followed by at least one `$y,`
  ($x:expr, $($y:expr),+) => (
      // Call `find_min!` on the tail `$y`
      std::cmp::min($x, find_min!($($y),+))
  )
}

fn main() {
  println!("{}", find_min!(1u32));
  println!("{}", find_min!(1u32 + 2, 2u32));
  println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
