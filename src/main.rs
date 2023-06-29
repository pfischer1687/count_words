use count_words::get_most_common_words;
use std::{fs::File, io::BufReader, io::prelude::*};

fn main() {
    let file: Result<File, std::io::Error> = File::open("txt/test.txt");
    let mut contents: String = String::new();
    let n: usize = 2;
    let mut result: Vec<(String, i32)> = Vec::new();

    let mut buf_reader: BufReader<File> = match file {
        Ok(file) => BufReader::new(file),
        Err(e) => { panic!("Error: {e}"); },
    };

    let did_contents_buffer: Result<usize, std::io::Error> = buf_reader.read_to_string(&mut contents);

    match did_contents_buffer {
        Ok(_) => { get_most_common_words(n, &contents, &mut result) },
        Err(e) => { panic!("Error: {e}"); },
    };

    println!("{:?}", result);
}
