impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        if n == 0 {
            return "".to_string();
        }

        let mut dp = vec![vec![false; n]; n];
        let mut start = 0;
        let mut max_len = 1;

        for i in 0..n {
            dp[i][i] = true; // un seul caractère
        }

        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;

                if chars[i] == chars[j] {
                    if len == 2 {
                        dp[i][j] = true; // deux mêmes caractères
                    } else {
                        dp[i][j] = dp[i + 1][j - 1]; // plus long palindrome interne
                    }

                    if dp[i][j] && len > max_len {
                        start = i;
                        max_len = len;
                    }
                }
            }
        }

        chars[start..start + max_len].iter().collect()
    }
}