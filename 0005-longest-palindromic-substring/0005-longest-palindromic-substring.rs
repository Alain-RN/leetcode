impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let input: String = s;
        if input == input.chars().rev().collect::<String>() {
            input
        } else {
            let chars: Vec<(usize, char)> = input.char_indices().collect();
            let mut sub_pal = String::from(chars[0].1);
            for start in 0..chars.len() {
                for end in start+1..=chars.len() {
                    let i = chars[start].0;
                    let j = if end < chars.len() { chars[end].0 } else { input.len() };
                    let substring = &input[i..j];
                    if substring == substring.to_string().chars().rev().collect::<String>() && substring.to_string().len() > sub_pal.len()  {
                        sub_pal = substring.to_string();
                    }
                }
            }

            sub_pal
        }
    }
}