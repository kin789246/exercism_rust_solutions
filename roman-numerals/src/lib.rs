use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    dig: usize,
    ten: usize,
    hun: usize,
    tho: usize,
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let digs = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        let tens = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX","LXXX", "XC"];
        let huns = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        let thos = vec!["", "M", "MM", "MMM"];
        
        let s = [
            thos[self.tho].to_string(),
            huns[self.hun].to_string(),
            tens[self.ten].to_string(),
            digs[self.dig].to_string(),
        ].concat();
        write!(_f, "{}", s)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut i = 0;
        let mut n = vec! [0, 0, 0, 0];
        let mut x = num;
        while x > 0 {
            n[i] = (x % 10) as usize;
            x /= 10;
            i += 1;
        }
        Self { dig: n[0], ten: n[1], hun: n[2], tho: n[3] }
    }
}
