use count_words::count_words;
use std::collections::HashMap;

fn main() {
    let hello: String = String::from("Hello, world!\nWorld says hello.");
    let mut word_counter: HashMap<String, i32> = HashMap::new();
    count_words(&hello, &mut word_counter);
    println!("{:?}", word_counter);
}
