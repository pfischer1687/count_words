// cargo run txt/opticks.txt

use count_words::get_most_common_words;
use std::{fs::File, io::BufReader, io::prelude::*, env};

fn main() -> std::io::Result<()> {
    let text_file: String = match env::args().nth(1) {
        Some(text_file) => text_file,
        None => { panic!("Error: text file name command line argument required"); },
    };

    let mut contents: String = String::new();
    let n: usize = 10;
    let mut result: Vec<(String, i32)> = Vec::new();

    let file: File = File::open(text_file)?;
    let mut buf_reader: BufReader<File> = BufReader::new(file);
    buf_reader.read_to_string(&mut contents)?;

    get_most_common_words(n, &contents, &mut result);
    println!("{:?}", result);
    Ok(())
}
