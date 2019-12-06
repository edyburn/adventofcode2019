fn has_pair(s: &[char]) -> bool {
    for i in 0..(s.len() - 1) {
        if s[i] == s[i + 1] {
            if (i > 0) && (s[i - 1] == s[i]) {
                continue;
            }
            if (i < (s.len() - 2)) && (s[i] == s[i + 2]) {
                continue;
            }
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
    // Two adjacent digits are the same but not part of a larger group of matching digits
    // Going from left to right, the digits never decrease
    has_pair(&s) & is_increasing(&s)
}

fn main() {
    #[allow(clippy::unreadable_literal)]
    let valid_count = (272091..815432).filter(|v| validate(*v)).count();
    println!("result: {}", valid_count);
}
