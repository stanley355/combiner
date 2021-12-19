mod collection;
use collection::{find_mean, find_median, find_mode, to_pig};

fn main() {
    let arr =vec![6, 3, 7, 8, 8, 8]; 

    find_mean(arr.clone());
    find_median(arr.clone());
    find_mode(arr.clone());
    to_pig("first");
}
