fn main() {
    let x = 5;
    println!("X value is {x}");

    let x = 8;
    println!("X values is {x}");

    {
        let x = x * 30;
        println!("X value is {x}");
    }

    println!("X values is {x}");
}
