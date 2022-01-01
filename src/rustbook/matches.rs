enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

// with default placeholder _
pub fn what_ever() {
  let dice_roll = 9;
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
  }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
