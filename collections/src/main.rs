//use std::fmt;
//
//fn main() {
//    enum SpreadsheetCell {
//        Int(i32),
//        Float(f64),
//        Text(String),
//    }
//
//    impl fmt::Display for SpreadsheetCell {
//        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//            match self {
//                SpreadsheetCell::Int(i) => write!(f, "{i}"),
//                SpreadsheetCell::Float(fl) => write!(f, "{fl}"),
//                SpreadsheetCell::Text(s) => write!(f, "{s}"),
//            }
//        }
//    }
//
//    let row = vec![
//        SpreadsheetCell::Int(3),
//        SpreadsheetCell::Text(String::from("blue")),
//        SpreadsheetCell::Float(10.12),
//    ];
//
//    for elm in &row {
//        println!("{elm}");
//    }
//}
//

fn main() {
    let a = "Здравствуйте";
    for i in a.bytes() {
        let b = i as char;
        println!("{b}");
    }
    println!("{a}");
}
