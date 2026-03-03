#[derive(Debug)]
struct User {
    name: String,
    age: usize,
}

fn main() {
    let user1 = User {
        name: String::from("Rael"),
        age: 14,
    };

    println!("{:#?}", user1);

    let user2 = User { ..user1 };

    println!("{:#?}", user2.name);
}
