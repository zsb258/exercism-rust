/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .try_rfold((0, 0), |(i, sum), c| {
            c.to_digit(10)
                .map(|n| {
                    if is_odd_pos(i) {
                        n
                    } else {
                        match n {
                            9 => 9,
                            x => (x * 2) % 9,
                        }
                    }
                })
                .map(|n| (i + 1, sum + n))
        })
        .map_or(false, |(count, sum)| count > 1 && sum % 10 == 0)
}

fn is_odd_pos(i: usize) -> bool {
    i % 2 == 0
}
