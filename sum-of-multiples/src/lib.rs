pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|n| {
            factors
                .iter()
                .find(|&&x| n.checked_rem(x).is_some_and(|r| r == 0))
                .is_some()
        })
        .sum()
}
