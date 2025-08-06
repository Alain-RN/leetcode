impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut v = vec![String::from(""); num_rows as usize];
        let mut etage = 0;
        let mut m = true;
        for c in s.chars() {
            v[etage].push(c);
            if m {
                etage += 1;
            } else {
                etage -= 1;
            }

            if etage == 0 || etage == (num_rows - 1) as usize {
                m = !m;
            }
        }
        v.join("")
    }
}