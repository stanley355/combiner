mod slice;
use slice::first_word;

fn main() {
    let mut s = String::from("Hello world");

    let word = first_word(&s);
    
    println!("{:?}", word);
}
