fn main() {
    println!("Enter Main Function");
    let y = ext_func(8);
    println!("{y}");
}

fn ext_func(x: i32) -> i32 {
    println!("Enter Ext Function {x}");
    4
}
