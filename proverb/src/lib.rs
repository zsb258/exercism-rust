pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        String::new()
    } else {
        let mut verses: Vec<_> = list
            .windows(2)
            .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
            .collect();
        verses.push(format!("And all for the want of a {}.", list[0]));

        verses.join("\n")
    }
}
