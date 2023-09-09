use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = sorted(&word_lower);

    possible_anagrams
        .iter()
        .filter_map(|pa| {
            let pa_lower = pa.to_lowercase();
            if pa_lower != word_lower && word_sorted == sorted(&pa_lower) {
                Some(*pa)
            } else {
                None
            }
        })
        .clone()
        .collect()
}

fn sorted(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}
