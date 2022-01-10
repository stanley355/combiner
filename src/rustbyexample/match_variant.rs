fn main() {
  let triple = (0, -2, 3);
  // TODO ^ Try different values for `triple`

  println!("Tell me about {:?}", triple);
  // Match can be used to destructure a tuple
  match triple {
      // Destructure the second and third elements
      (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
      (1, ..)  => println!("First is `1` and the rest doesn't matter"),
      // `..` can be used to ignore the rest of the tuple
      _      => println!("It doesn't matter what they are"),
      // `_` means don't bind the value to a variable
  }
}

// `allow` required to silence warnings because only
// one variant is used.
#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }
}

fn main() {
  struct Foo {
      x: (u32, u32),
      y: u32,
  }

  // Try changing the values in the struct to see what happens
  let foo = Foo { x: (1, 2), y: 3 };

  match foo {
      Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

      // you can destructure structs and rename the variables,
      // the order is not important
      Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

      // and you can also ignore some variables:
      Foo { y, .. } => println!("y = {}, we don't care about x", y),
      // this will give an error: pattern does not mention field `x`
      //Foo { y } => println!("y = {}", y),
  }
}
