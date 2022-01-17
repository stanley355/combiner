use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let t = start.date();
    t.checked_add(1000000000.seconds())
    PrimitiveDateTime::new(date!(2019-01-01), time!(0:00))
}

fn main() {
    println!("Hello");
}
