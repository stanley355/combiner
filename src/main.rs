enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: usize, y: usize) -> usize {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
    let c = Operations::Add;
    let d = c.run(3, 2);
    println!("{}", d);
}
