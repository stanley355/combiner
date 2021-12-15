mod structure;
use structure::{build_user, User};

fn main() {
    let em = String::from("hofwof");
    let us = String::from("hofwoffawfw");

    let user1 = build_user(em, us); 
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };   
    println!("{:?}", user2);
}
