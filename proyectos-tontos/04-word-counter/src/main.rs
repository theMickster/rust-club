use std::fs;
use std::collections::HashMap;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() < 2 {
        println!("Please provide a filename as a command line argument.");
        return;
    }

    let filename = &arguments[1];
    let contents = read_file(filename);
    let words = tokenize(&contents);
    let word_count = count_words(&words); 
    let sorted_words = sort_descending(&word_count);

    println!("Total word count: {}", words.len());

    println!("\nWord Frequencies (sorted by frequency descending):");
    for(word, count) in sorted_words.iter().take(25) {
        println!("{:<15} {}", word, count);
    }

    print!("Thank you for using the word frequency counter");
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("An error occurred with reading your file... Please try again")
}

/// Tokenizes the input text into words, removing punctuation but preserving letters, 
/// digits, apostrophes, hyphens and converting to lowercase.
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| {
            word.chars()
                .filter(|c| c.is_alphanumeric() || *c == '\'' || *c == '-')
                .collect::<String>()
                .to_lowercase()
        })
        .filter(|w| !w.is_empty())
        .collect()
}

/// Counts the occurrences of each word in the provided slice of words.
fn count_words (words: &[String]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in words {
        *map.entry(word.clone()).or_insert(0) += 1;
    }
    map
}

/// Sort hashMap of words by frequency in descending order
fn sort_descending(map: &HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut vec: Vec<(String, usize)> = map.iter()
        .map(|(word, count)| (word.clone(), *count))
        .collect();

    vec.sort_by( | a, b | b.1.cmp(&a.1).then_with(||a.0.cmp(&b.0)));
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    /// Simple helper to convert string slice to Vec<String> for cleaner test cases
    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    /// Simple helper to create expected HashMap from tuples
    fn expected(pairs: &[(&str, usize)]) -> HashMap<String, usize> {
        pairs.iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect()
    }

    #[rstest]
    #[case::empty(vec![], expected(&[]))]
    #[case::single_word(strings(&["hello"]), expected(&[("hello", 1)]))]
    #[case::two_different_words(
        strings(&["hello", "world"]), 
        expected(&[("hello", 1), ("world", 1)])
    )]
    #[case::duplicate_words(
        strings(&["rust", "rust", "rust"]), 
        expected(&[("rust", 3)])
    )]
    fn test_count_words(#[case] input: Vec<String>, #[case] expected: HashMap<String, usize>) {
        let result = count_words(&input);
        assert_eq!(result, expected);
    }
}