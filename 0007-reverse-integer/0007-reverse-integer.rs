impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x < 0 { -1 } else { 1 };
        let mut x = x.abs();
        let mut rev: i32 = 0;

        while x > 0 {
            let digit = x % 10;
            x /= 10;

            if rev > (i32::MAX - digit) / 10 {
                return 0;
            }

            rev = rev * 10 + digit;
        }

        rev * sign
    }
}