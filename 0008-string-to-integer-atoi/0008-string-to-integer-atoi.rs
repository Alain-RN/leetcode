impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut clean = String::new();
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return 0;
        }

        let mut sign = 1;
        let mut start = 0;
        match trimmed.chars().nth(0) {
            Some('-') => {
                sign = -1;
                start = 1;
            }
            Some('+') => {
                start = 1;
            }
            _ => {}
        }
    
        let mut digit: i64 = 0;
        for c in trimmed[start..].chars() {
            if let Some(d) = c.to_digit(10) {
                digit = digit * 10 + d as i64;

                if sign == 1 && digit > i32::MAX as i64 {
                    return i32::MAX;
                }
                if sign == -1 && -digit < i32::MIN as i64 {
                    return i32::MIN;
                }
            } else {
                break;
            }
        }

        (digit as i32) * sign
    }
}