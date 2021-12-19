use std::collections::HashMap;

// mean, sorted median, and modues
pub fn find_mean(arr: Vec<usize>) {
  let lent = arr.len();
  let mut bo = 0;

  for x in arr {
    bo += x;
  }

  let result = bo / lent;
  println!("The mean is : {:?}", result);
}

pub fn find_median(arr: Vec<usize>) {
  let mut bar = arr.clone();
  bar.sort();

  let med:usize;
  let mid = (bar.len() / 2) - 1;

  if bar.len() % 2 == 1 {
    med = bar[mid + 1];
  } else {
    med = (bar[mid] + bar[mid + 1]) / 2;
  }

  println!("The sorted array is: {:?}", bar);
  println!("The sorted median is: {:?}", med);
}

pub fn find_mode(arr: Vec<usize>) {
  let bar = arr.clone();
  let mut map = HashMap::new();

  for word in bar {
      let count = map.entry(word).or_insert(0);
      *count += 1;
  }

  let result = map.iter().max_by(|a, b| a.cmp(b)).map(|(k, _v)| k).unwrap();

  println!("The mode is : {:?}", result);
}
