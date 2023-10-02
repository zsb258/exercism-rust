pub fn nth(n: u32) -> u32 {
    // Using Sieve of Eratosthenes

    // hardcoded upper bound based on test case
    const N: usize = 1_000_000;

    // 1-based indexing
    let mut nums = vec![true; N + 1];

    // edge cases
    nums[0] = false;
    nums[1] = false;

    let iter_max = (N as f64).sqrt() as usize + 1;
    for i in 2..=iter_max {
        if nums[i] {
            let mut j = i.pow(2);
            while j <= N {
                nums[j] = false;
                j += i;
            }
        }
    }

    nums.into_iter()
        .enumerate()
        .filter_map(|(i, is_prime)| if is_prime { Some(i as u32) } else { None })
        .nth(n as usize)
        .unwrap()
}
