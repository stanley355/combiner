pub fn printype() {
  println!("I'm a rustacean");
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
  println!(
    "{subject} {verb} {object}",
    object = "the lazy dog",
    subject = "the quick brown fox",
    verb = "jumps over"
  );

  #[derive(Debug)]
  struct Structure(i32);
  println!("This struct {:?} won't print...", Structure(3));

  let pi = 3.141592;
  println!("Pi is roughly {:.2}", pi); //print 3.14
}
