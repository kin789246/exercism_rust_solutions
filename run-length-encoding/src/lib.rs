pub fn encode(source: &str) -> String {
    if source.len() == 0 { return "".to_string() }
    let mut ans: String = String::new();
    let mut p: usize = 0;
    let mut ch: char = source.chars().nth(0).unwrap();
    for (i, c) in source.chars().enumerate() {
        if c != ch {
            let repeat: usize = i-p;
            if repeat == 1 { 
                ans.push(ch) 
            }
            else {
                ans += &repeat.to_string();
                ans.push(ch);
            }
            ch = c;
            p = i;
        }
        if i == source.len()-1 {
            let repeat: usize = i+1-p;
            if repeat == 1 { 
                ans.push(ch) 
            }
            else {
                ans += &repeat.to_string();
                ans.push(ch);
            }
        }
    }
    ans
}

pub fn decode(source: &str) -> String {
    if source.len() == 0 { return "".to_string() }
    let mut ans = String::new();
    let mut num: String = String::new();
    for c in source.chars() {
        if c.is_digit(10) {
            num.push(c);
        }
        else {
            if num.len() == 0 {
                ans.push(c);
            }
            else {
                ans += &c.to_string().repeat(num.parse().unwrap());
            }
            num = "".to_string();
        }
    }
    ans
}
