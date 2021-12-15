pub struct Rectangle {
  pub width: usize,
  pub height: usize,
}


pub fn area(rectangle: &Rectangle) -> usize {
  rectangle.width * rectangle.height
}
