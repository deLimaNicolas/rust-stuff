fn main() {
    {
        let mut s: String = String::from("aiai");
        println!("{}-blubla", s);
        s.push_str(", atleticoo");
        println!("{}", s);
        let s2 = s;
        println!("{}-s2", s2);
    }
}
