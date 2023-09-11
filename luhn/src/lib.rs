/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if code.len() <= 1 {
        false
    } else {
        is_valid_impl(code)
    }
}

fn is_odd_pos(i: usize) -> bool {
    i % 2 == 0
}

fn is_valid_impl(code: String) -> bool {
    let num_options = code.chars().map(|c| c.to_digit(10));

    if num_options.clone().any(|x| x.is_none()) {
        return false;
    }

    let checksum = num_options
        .map(|x| x.unwrap())
        .rev()
        .enumerate()
        .map(|(i, n)| {
            if is_odd_pos(i) {
                n
            } else {
                match n {
                    9 => 9,
                    x => (x * 2) % 9,
                }
            }
        })
        .fold(0, |acc, n| acc + n);

    if checksum % 10 == 0 {
        true
    } else {
        false
    }
}
