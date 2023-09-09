#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if issub(first_list, second_list) {
        Comparison::Sublist
    } else if issub(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn issub<T: PartialEq>(short_list: &[T], long_list: &[T]) -> bool {
    if short_list.is_empty() {
        true
    } else {
        long_list.windows(short_list.len()).any(|w| w == short_list)
    }
}
