mod matches;
use matches::plus_one;

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Can rect1 hold rect2? {:?}", six);
    println!("Can rect1 hold rect3? {:?}", none);
}
