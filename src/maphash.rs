use std::collections::HashMap;

pub fn maphashing() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  println!("this is {:?}", scores);
}

pub fn mapping() {
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

  println!("this is {:?}", scores);
}
