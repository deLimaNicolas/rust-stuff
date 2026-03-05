#[derive(Debug)]
struct Rectangle {
    height: usize,
    width: usize,
}

impl Rectangle {
    fn get_area(&self) -> usize {
        self.height * self.width
    }

    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.height >= rec.height && self.width >= rec.width
    }
}

fn main() {
    let rec = Rectangle {
        height: 40,
        width: 2,
    };

    let rec2 = Rectangle {
        height: 40,
        width: 2,
    };

    let rec3 = Rectangle {
        height: 90,
        width: 6,
    };

    println!("Rec1, Rec2 {}", rec.can_hold(&rec2));
    println!("Rec1, Rec3 {}", rec.can_hold(&rec3));
    println!("Rec3, Rec1 {}", rec3.can_hold(&rec));
}
