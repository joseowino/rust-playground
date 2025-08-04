pub fn reverse(x: i32) -> i32 {
    let sign = if x < 0 { -1 } else { 1 };
    let reversed_str: String = x.abs().to_string().chars().rev().collect();

    match reversed_str.parse::<i32>() {
        Ok(num) => sign * num,
        Err(_) => 0, // Return 0 on overflow
    }
}

