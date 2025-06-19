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

    /// Simple helper to create HashMap from tuples
    fn word_map(pairs: &[(&str, usize)]) -> HashMap<String, usize> {
        pairs.iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect()
    }

    /// Simple helper to create expected Vec from tuples
    fn expected_sort(pairs: &[(&str, usize)]) -> Vec<(String, usize)> {
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
    #[case::mixed_duplicates(
        strings(&["foo", "bar", "foo", "baz", "bar", "foo"]),
        expected(&[("foo", 3), ("bar", 2), ("baz", 1)])
    )]
    #[case::case_sensitive(
        strings(&["Rust", "rust", "RUST"]),
        expected(&[("Rust", 1), ("rust", 1), ("RUST", 1)])
    )]
    #[case::special_characters(
        strings(&["hello!", "hello!", "world?"]),
        expected(&[("hello!", 2), ("world?", 1)])
    )]    
    fn test_count_words(#[case] input: Vec<String>, #[case] expected: HashMap<String, usize>) {
        let result = count_words(&input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::empty("", vec![])]
    #[case::single_word("hello", strings(&["hello"]))]
    #[case::multiple_words("hello world rust", strings(&["hello", "world", "rust"]))]
    #[case::mixed_case("Hello World RUST", strings(&["hello", "world", "rust"]))]
    #[case::extra_spaces("hello    world", strings(&["hello", "world"]))]
    #[case::leading_trailing_spaces("  hello world  ", strings(&["hello", "world"]))]
    #[case::tabs_and_newlines("hello\tworld\nrust", strings(&["hello", "world", "rust"]))]
    #[case::punctuation_removal("hello, world! rust?", strings(&["hello", "world", "rust"]))]
    #[case::trailing_punctuation("hello... world!!!", strings(&["hello", "world"]))]
    #[case::leading_punctuation("...hello !!!world", strings(&["hello", "world"]))]
    #[case::contractions_with_apostrophes(
        "don't can't won't it's",
        strings(&["don't", "can't", "won't", "it's"])
    )]
    #[case::hyphenated_words(
        "well-known state-of-the-art",
        strings(&["well-known", "state-of-the-art"])
    )]
    #[case::mixed_apostrophes_and_hyphens(
        "it's a well-known fact",
        strings(&["it's", "a", "well-known", "fact"])
    )]
    #[case::numbers("hello123 world456", strings(&["hello123", "world456"]))]
    #[case::only_numbers("123 456", strings(&["123", "456"]))]
    #[case::punctuation_only("!!! ??? ...", vec![])]
    #[case::mixed_punctuation_and_text(
        "hello!!! @world #rust",
        strings(&["hello", "world", "rust"])
    )]
    #[case::apostrophes_at_boundaries(
        "'hello' 'world'",
        strings(&["'hello'", "'world'"])
    )]
    #[case::hyphens_at_boundaries(
        "-hello- -world-",
        strings(&["-hello-", "-world-"])
    )]
    #[case::complex_sentence(
        "It's a well-known fact that Rust's memory-safety is top-notch!",
        strings(&["it's", "a", "well-known", "fact", "that", "rust's", "memory-safety", "is", "top-notch"])
    )]
    #[case::unicode_handling(
        "hello→world café naïve",
        strings(&["helloworld", "café", "naïve"])
    )]
    #[case::special_characters(
        "hello@world.com #rust $100",
        strings(&["helloworldcom", "rust", "100"])
    )]
    #[case::multiple_hyphens_and_apostrophes(
        "don't-stop-believing it's-a-me",
        strings(&["don't-stop-believing", "it's-a-me"])
    )]
    fn test_tokenize(#[case] input: &str, #[case] expected: Vec<String>) {
        let result = tokenize(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::empty(
        word_map(&[]),
        expected_sort(&[])
    )]
    #[case::single_entry(
        word_map(&[("rust", 5)]),
        expected_sort(&[("rust", 5)])
    )]
    #[case::descending_by_count(
        word_map(&[("rust", 10), ("python", 5), ("java", 2)]),
        expected_sort(&[("rust", 10), ("python", 5), ("java", 2)])
    )]
    #[case::tie_breaking_alphabetical(
        word_map(&[("zebra", 5), ("apple", 5), ("banana", 5)]),
        expected_sort(&[("apple", 5), ("banana", 5), ("zebra", 5)])
    )]
    #[case::mixed_counts_and_ties(
        word_map(&[
            ("rust", 10),
            ("python", 7),
            ("java", 7),
            ("go", 3),
            ("c", 3),
            ("ruby", 1)
        ]),
        expected_sort(&[
            ("rust", 10),
            ("java", 7),
            ("python", 7),
            ("c", 3),
            ("go", 3),
            ("ruby", 1)
        ])
    )]
    fn test_sort_descending(
        #[case] input: HashMap<String, usize>,
        #[case] expected: Vec<(String, usize)>
    ) {
        let result = sort_descending(&input);
        assert_eq!(result, expected);
    }

}