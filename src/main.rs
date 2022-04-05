use std::mem;

#[derive(Debug)]
enum User {
    Reader { name: String },
    Writer { name: String },
    Admin { name: String }
}

fn promote(u: &mut User) {
    use User::*;

    *u = match u {
        Reader { name } => Writer { name: mem::take(name) },
        Writer { name } => Admin { name: mem::take(name) },
        Admin { name: _ } => return,
    }
}

fn main() {
    let mut user = User::Reader{ name: "Wallace".to_owned() };
    println!("{user:?}");

    promote(&mut user);
    println!("{user:?}");

    promote(&mut user);
    println!("{user:?}");
}

