#[derive(Debug)]

pub struct Rectangle {
  pub width: usize,
  pub height: usize,
}

impl Rectangle {
  fn square(size: usize) -> Rectangle {
    Rectangle {
        width: size,
        height: size,
    }
  }

  pub fn area(&self) -> usize {
    self.width * self.height
  }

  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}
