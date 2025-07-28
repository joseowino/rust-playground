use std::collections::HashMap;

fn main() {
    let s = "pwwkew".to_string();
    let result = length_of_longest_substring(s);
    println!("result: {}", result);
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut map = HashMap::new();
    let mut start = 0;
    let mut max_len = 0;

    for (i, c) in s.chars().enumerate() {
        if let Some(&prev_index) = map.get(&c) {
            if prev_index >= start {
                start = prev_index + 1;
            }
        }

        map.insert(c, i);
        max_len = max_len.max(i - start + 1);
    }

    max_len as i32
}

