fn main() {
  // Increment via closures and functions.
  fn function(i: i32) -> i32 { i + 1 }

  // Closures are anonymous, here we are binding them to references
  // Annotation is identical to function annotation but is optional
  // as are the `{}` wrapping the body. These nameless functions
  // are assigned to appropriately named variables.
  let closure_annotated = |i: i32| -> i32 { i + 1 };
  let closure_inferred  = |i     |          i + 1  ;

  let i = 1;
  // Call the function and closures.
  println!("function: {}", function(i));
  println!("closure_annotated: {}", closure_annotated(i));
  println!("closure_inferred: {}", closure_inferred(i));

  // A closure taking no arguments which returns an `i32`.
  // The return type is inferred.
  let one = || 1;
  println!("closure returning one: {}", one());

}

// Using move before vertical pipes forces closure to take ownership of captured variables:
fn main() {
  // `Vec` has non-copy semantics.
  let haystack = vec![1, 2, 3];

  let contains = move |needle| haystack.contains(needle);

  println!("{}", contains(&1));
  println!("{}", contains(&4));

  // println!("There're {} elements in vec", haystack.len());
  // ^ Uncommenting above line will result in compile-time error
  // because borrow checker doesn't allow re-using variable after it
  // has been moved.
  
  // Removing `move` from closure's signature will cause closure
  // to borrow _haystack_ variable immutably, hence _haystack_ is still
  // available and uncommenting above line will not cause an error.
}
