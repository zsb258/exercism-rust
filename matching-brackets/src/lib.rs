pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        if !match c {
            ')' => stack.pop().is_some_and(|c| c == '('),
            ']' => stack.pop().is_some_and(|c| c == '['),
            '}' => stack.pop().is_some_and(|c| c == '{'),
            '(' | '[' | '{' => {
                stack.push(c);
                true
            }
            _ => true,
        } {
            return false;
        }
    }

    stack.is_empty()
}
