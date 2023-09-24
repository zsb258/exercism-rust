pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u64> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let n = digits.len() as u32;

    digits.into_iter().fold(0_u64, |mut acc, d| {
        acc += d.pow(n);
        acc
    }) == num as u64
}
