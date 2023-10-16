pub struct RailFence {
    rows: usize,
} 

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self { rows: rails as usize }
    }

    pub fn encode(&self, text: &str) -> String {
        let columns = text.chars().count();
        let mut row = 0i32;
        let mut dr = 1i32;
        let mut fence = vec![vec![' '; columns]; self.rows];
        for col in 0..columns {
            fence[row as usize][col] = text.chars().nth(col).unwrap();
            row = row + dr;
            if row == self.rows as i32 - 1 {
                dr = -1;
            }
            if row == 0 {
                dr = 1;
            }
        }

        fence.iter().flat_map(|arr| arr.iter().filter(|c| !c.is_whitespace()))
            .collect::<String>()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let columns = cipher.chars().count();
        let mut row = 0i32;
        let mut dr = 1i32;
        let mut fence = vec![vec![' '; columns]; self.rows];
        for col in 0..columns {
            fence[row as usize][col] = '?';
            row = row + dr;
            if row == self.rows as i32 - 1 {
                dr = -1;
            }
            if row == 0 {
                dr = 1;
            }
        }
        let mut dc = 0usize;
        for r in 0..self.rows {
            for c in 0..columns {
                if fence[r][c] == '?' {
                    fence[r][c] = cipher.chars().nth(dc).unwrap();
                    dc += 1;
                }
            }
        }
        let mut res = String::new();
        for c in 0..columns {
            for r in 0..self.rows {
                if fence[r][c] != ' ' {
                    res.push(fence[r][c]);
                }
            }
        }
        res
    }
}
