
pub fn find_mean(arr: Vec<usize>) {
  let row = arr;
  let lent = row.len();
  let mut bo = 0;

  for x in row {
    bo += x;
  }
  println!("{}", bo);
  println!("{}", lent);
  println!("{:?}", bo / lent);
}
