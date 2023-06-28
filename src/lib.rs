use std::collections::HashMap;

pub fn count_words(s: &str, word_counter: &mut HashMap<String, i32>) {
    let s_split: std::str::SplitWhitespace<'_> = s.split_whitespace();
    for word in s_split {
        let filtered_word: String = filter_letters(word);
        word_counter.entry(filtered_word).and_modify(|number: &mut i32| *number += 1).or_insert(1);
    }
}

fn filter_letters(word: &str) -> String {
    let mut result: String = String::new();
    for c in word.chars() {
        if !c.is_ascii_alphabetic() {
            continue;
        }

        result.push(c.to_ascii_lowercase());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_letters() {
        let hello: String = String::from("123[]<>()Hello,! ");
        let result:String  = filter_letters(&hello);
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
}