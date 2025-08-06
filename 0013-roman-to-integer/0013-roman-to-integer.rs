impl Solution {

    pub fn roman_to_int(s: String) -> i32 {
        let eq_val: Vec<(char, u32)> = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ];

        let mut prev = s.chars().nth(s.chars().count() - 1).unwrap();
        let mut val_prev = 0;

        if let Some((_, val)) = eq_val.iter().find(|(ch, _)| *ch == prev) {
            val_prev = *val;
        }

        let mut val_tot = val_prev;

        for r in s[..s.len() - 1].chars().rev() {
            if let Some((_, val)) = eq_val.iter().find(|(ch, _)| *ch == r) {
                if val_prev > *val {
                    val_tot -= *val;
                } else {
                    val_tot += *val;
                }

                val_prev = *val;
            }
        }
        val_tot as i32
    }
}