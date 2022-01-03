#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
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
    }
}

fn main() {
    let point: Point = Point { x: 2.3, y: 0.4 };

    let bot = Point { x: 4.5, ..point };

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { ..point },
        bottom_right: bot,
    };

    println!("second point: ({:?})", _rectangle.rect_area());
}
