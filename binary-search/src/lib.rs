use std::cmp::Ordering;

pub fn find<T: Ord, R: AsRef<[T]>>(array: R, key: T) -> Option<usize> {
    solve_by_loop(array, key)
    // _solve_by_iter(array, key)
}

fn solve_by_loop<T: Ord, R: AsRef<[T]>>(array: R, key: T) -> Option<usize> {
    let mut lt = 0usize;
    let mut rt = array.as_ref().len();
    while lt < rt {
        let mid = (lt + rt) / 2;
        match key.cmp(&array.as_ref()[mid]) {
            Ordering::Less => rt = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => lt = mid + 1,
        }
    }

    None
}

fn _solve_by_iter<T: Ord, R: AsRef<[T]>>(array: R, key: T) -> Option<usize> {
    array
        .as_ref()
        .iter()
        .enumerate()
        .find_map(|(i, &ref val)| if key == *val { Some(i) } else { None })
}
