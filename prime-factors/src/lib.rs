pub fn factors(mut n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut candidates = 2..=n;
    while n > 1 {
        let candidate = candidates.next().unwrap();
        while n > 1 && n % candidate == 0 {
            n = n / candidate;
            res.push(candidate);
        }
    }

    res
}
