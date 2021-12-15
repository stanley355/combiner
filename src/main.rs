mod taple;
use taple::{area, Rectangle};

fn main() {
    let rect1 =  Rectangle {
        width: 30,
        height: 50,
    };

    println!("{:?}", area(&rect1));
}
