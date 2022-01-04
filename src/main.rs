#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) {
        let area = self.top_left.x * self.bottom_right.y;
        println!("The size of area is {}", area);
    }
}

fn main() {
    let point: Point = Point { x: 2, y: 4 };

    let bot = Point { x: 5, ..point };

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { ..point },
        bottom_right: bot,
    };

    _rectangle.rect_area();
}
