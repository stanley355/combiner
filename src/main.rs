mod collection;
use collection::{find_mean, find_median};

fn main() {
    let arr =vec![6, 3, 7, 5, 9, 8]; 

    find_mean(arr.clone());
    find_median(arr.clone());
}
