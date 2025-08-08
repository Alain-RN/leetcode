impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut l: usize = 0;
        let mut rom: Vec<String> = Vec::new();

        let r_val: Vec<char> = vec!['I', 'V', 'X', 'L', 'C', 'D', 'M'];

        while num > 0 {
            let mut last = num % 10;
            let mut s = String::new();

            if last == 9 {
                s.push(r_val[l]);
                s.push(r_val[l + 2]);
            } else if last >= 5 {
                s.push(r_val[l + 1]);
                for _ in 0..(last - 5) {
                    s.push(r_val[l]);
                }
            } else if last == 4 {
                s.push(r_val[l]);
                s.push(r_val[l + 1]);
            } else {
                for _ in 0..last {
                    s.push(r_val[l]);
                }
            }

            rom.push(s);
            num /= 10;
            l += 2;
        }

        rom.reverse();
        rom.join("")
    }
}
