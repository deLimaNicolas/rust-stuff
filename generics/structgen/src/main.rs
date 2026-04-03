struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let a = Point { x: 2, y: 3 };

    let b = Point { x: "a", y: "b" };

    println!("here {} and here {}", a.get_x(), b.get_x());
}
