fn main() {
    let tup: (u32, f64, &str) = (45, 45.87, "45");
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("Values {x}, {y}, {z}");
}
