use std::{collections::HashMap, cmp::Ordering};

pub fn get_most_common_words(mut n: usize, s: &str, result: &mut Vec<(String, i32)>) {
    let mut sorted_words: Vec<(String, i32)> = Vec::new();
    let mut word_counter: HashMap<String, i32> = HashMap::new();
    count_words(&s, &mut word_counter);

    if word_counter.len() < 1 { return; }
    if word_counter.len() < n { n = word_counter.len(); }

    for (key, val) in word_counter.drain() {
        sorted_words.push((key, val));
    }

    sorted_words.sort_unstable_by(compare_words);

    for (key, val) in sorted_words.drain(..n) {
        result.push((key, val));
    }
}

fn compare_words(a: &(String, i32), b: &(String, i32)) -> Ordering {
    if a.1 == b.1 {
        return a.0.cmp(&b.0); // return word keys in alphabetical order
    }

    b.1.cmp(&a.1) // return count values from greatest to least
}

fn count_words(s: &str, word_counter: &mut HashMap<String, i32>) {
    let s_split: std::str::SplitWhitespace<'_> = s.split_whitespace();

    for word in s_split {
        let filtered_word: String = filter_letters(word);
        if filtered_word.len() < 1 { continue; }
        word_counter.entry(filtered_word).and_modify(|number: &mut i32| *number += 1).or_insert(1);
    }
}

fn filter_letters(word: &str) -> String {
    let mut result: String = String::new();
    for c in word.chars() {
        if !c.is_ascii_alphabetic() { continue; }
        result.push(c.to_ascii_lowercase());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader, io::prelude::*};

    #[test]
    fn test_filter_letters() {
        let hello: String = String::from("123[]<>()Hello,! ");
        let result: String = filter_letters(&hello);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_count_words() {
        let hello: String = String::from("Hello, world!\nWorld says hello.");
        let mut word_counter: HashMap<String, i32> = HashMap::new();
        count_words(&hello, &mut word_counter);
        let check_hash: HashMap<String, i32> = HashMap::from([("world".to_string(), 2), ("says".to_string(), 1), ("hello".to_string(), 2)]);
        assert_eq!(word_counter, check_hash);
    }

    #[test]
    fn test_count_words_empty() {
        let empty: String = String::from("");
        let mut word_counter: HashMap<String, i32> = HashMap::new();
        count_words(&empty, &mut word_counter);
        let check_hash: HashMap<String, i32> = HashMap::new();
        assert_eq!(word_counter, check_hash);
    }

    #[test]
    fn test_get_most_used_words() {
        let n: usize = 2;
        let hello: String = String::from("Hello, world!\nWorld says hello.");
        let mut result: Vec<(String, i32)> = Vec::new();
        get_most_common_words(n, &hello, &mut result);
        let check_result: Vec<(String, i32)> = vec![("hello".to_string(), 2), ("world".to_string(), 2)];
        assert_eq!(result, check_result);
    }

    #[test]
    fn test_get_most_used_words_empty() {
        let n: usize = 2;
        let empty: String = String::from("");
        let mut result: Vec<(String, i32)> = Vec::new();
        get_most_common_words(n, &empty, &mut result);
        let check_result: Vec<(String, i32)> = Vec::new();
        assert_eq!(result, check_result);
    }

    #[test]
    fn test_get_most_used_words_less_unique_words_than_n() {
        let n: usize = 10;
        let empty: String = String::from("Hello, hello world!");
        let mut result: Vec<(String, i32)> = Vec::new();
        get_most_common_words(n, &empty, &mut result);
        let check_result: Vec<(String, i32)> = vec![("hello".to_string(), 2), ("world".to_string(), 1)];
        assert_eq!(result, check_result);
    }

    #[test]
    fn test_get_most_common_words_on_test_txt() {
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

        let check_contents: Vec<(String, i32)> = vec![("the".to_string(), 13), ("of".to_string(), 9)];
        assert_eq!(result, check_contents);
    }
}