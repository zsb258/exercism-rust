pub fn reply(message: &str) -> &str {
    let chars = message.chars().filter(|c| !c.is_whitespace());
    let last = chars.clone().last();
    let alphabetic: String = chars.filter(|c| c.is_alphabetic()).collect();

    if last.is_none() {
        "Fine. Be that way!"
    } else if !alphabetic.is_empty() && alphabetic.chars().all(|c| c.is_ascii_uppercase()) {
        if let Some('?') = last {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if let Some('?') = last {
        "Sure."
    } else {
        "Whatever."
    }
}
