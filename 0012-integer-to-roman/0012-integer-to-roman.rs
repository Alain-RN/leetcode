impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let symbols = [
            ("M", 1000), ("CM", 900), ("D", 500), ("CD", 400),
            ("C", 100),  ("XC", 90),  ("L", 50),  ("XL", 40),
            ("X", 10),   ("IX", 9),   ("V", 5),   ("IV", 4),
            ("I", 1),
        ];

        let mut result = String::with_capacity(20);

        for &(sym, val) in symbols.iter() {
            while num >= val {
                result.push_str(sym);
                num -= val;
            }
            if num == 0 {
                break;
            }
        }

        result
    }
}
