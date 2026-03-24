use std::collections::HashMap;

fn main() {
    //    let mut scores: HashMap<&str, usize> = HashMap::new();
    //    scores.insert("Blue", 20);
    //    scores.insert("Black", 50);
    //
    //    let score = scores.get("Blue");
    //    match score {
    //        Some(t) => println!("iga"),
    //        None => println!("Not Found"),
    //    }
    //
    //    for (key, val) in &scores {
    //        println!("{key} : {val}");
    //    }
    //
    //    let score = scores.get("Blue");
    //    match score {
    //        Some(t) => println!("iga"),
    //        None => println!("Not Found"),
    //    }
    let mut scores = HashMap::new();

    let text = "lets count this words count lets";

    for word in text.split_whitespace() {
        let count = scores.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{scores:?}");
}
