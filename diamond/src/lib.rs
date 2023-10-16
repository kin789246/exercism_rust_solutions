pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as u8 - b'A';
    let mut res: Vec<String> = Vec::new();
    let mut step = n;
    for i in 0..2*n+1 {
        let mut s: Vec<String> = Vec::new();
        s.push(String::from_utf8(vec![ b' '; step.into() ]).unwrap()); 
        let ch = (n - step + b'A') as char;
        s.push(String::from(ch));
        let m = 2 * (n as i16 - step as i16) - 1;
        if m >= 0 {
            s.push(String::from_utf8(vec![b' '; m as usize]).unwrap());
            s.push(String::from(ch));
        }
        s.push(String::from_utf8(vec![ b' '; step.into() ]).unwrap()); 
        res.push(s.concat());

        if i < n {
            step -= 1;
        }
        else {
            step += 1;
        }
    }

    res
}
