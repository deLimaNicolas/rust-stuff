fn main() {
    let mut word = String::from("test my love");
    let mut j = get_word_end(&word);
    word.clear();
    println!("{}", j);
}

fn get_word_end(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == ' ' as u8 {
            return &s[..i];
        }
    }
    &s[..s.len()]
}
