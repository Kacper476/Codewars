fn spin_words(words: &str) -> String {
    
    let rev_word = words.split_whitespace()
    .map(|word| {
        if word.len() >= 5 {
            word.chars().rev().collect()
        } else {
            word.to_string()
        }
    })
    .fold(String::new(), |mut acc, word| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(&word);
        acc
    });
    
    rev_word
}