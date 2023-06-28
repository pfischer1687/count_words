use count_words::get_most_used_words;

fn main() {
    let n: usize = 2;
    let hello: String = String::from("Hello, world!\nWorld says hello.");
    let mut result: Vec<(String, i32)> = Vec::new();
    get_most_used_words(n, &hello, &mut result);
    println!("{:?}", result);
}
