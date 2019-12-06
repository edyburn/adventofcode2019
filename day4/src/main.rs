fn has_pair(s: &[char]) -> bool {
    for i in 0..(s.len() - 1) {
        if s[i] == s[i + 1] {
            return true;
        }
    }
    false
}

fn is_increasing(s: &[char]) -> bool {
    for i in 0..(s.len() - 1) {
        if s[i] > s[i + 1] {
            return false;
        }
    }
    true
}

fn validate(n: u32) -> bool {
    let s: Vec<char> = n.to_string().chars().collect();
    // Two adjacent digits are the same
    // Going from left to right, the digits never decrease
    has_pair(&s) & is_increasing(&s)
}

fn main() {
    #[allow(clippy::unreadable_literal)]
    let valid_count = (272091..815432).filter(|v| validate(*v)).count();
    println!("result: {}", valid_count);
}
