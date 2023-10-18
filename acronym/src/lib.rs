pub fn abbreviate(phrase: &str) -> String {
    let chars: Vec<char> = " ".chars().chain(phrase.chars()).collect();
    chars
        .windows(2)
        .filter_map(|pair| {
            if is_abbrev_letter(pair[0], pair[1]) {
                Some(pair[1].to_ascii_uppercase())
            } else {
                None
            }
        })
        .collect()
}

fn is_abbrev_letter(prev: char, curr: char) -> bool {
    curr.is_alphabetic()
        && match prev {
            ' ' | '_' | '-' => true,
            _ => curr.is_ascii_uppercase() && prev.is_ascii_lowercase(),
        }
}
