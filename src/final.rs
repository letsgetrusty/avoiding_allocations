use std::{mem};

#[derive(Debug)]
enum User {
    Reader { name: String },
    Writer { name: String },
    Admin { name: String }
}

fn main() {
    let mut user = User::Reader{ name: "Wallace".to_owned() };
    println!("{user:?}");

    promote(&mut user);
    println!("{user:?}");

    promote(&mut user);
    println!("{user:?}");
}

fn promote(u: &mut User) {
    use User::*;

    // *u = match u {
    //     Reader { name } => Writer { name: name.clone() },
    //     Writer { name } => Admin { name: name.clone() },
    //     Admin { name: _ } => return,
    // }
    *u = match u {
        // this takes out our `name` and put in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will
        // be assigned to `*u`).
        Reader { name } => Writer { name: mem::take(name) },
        Writer { name } => Admin { name: mem::take(name) },
        Admin { name: _ } => return,
    }
}
