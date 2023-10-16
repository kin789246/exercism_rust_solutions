pub fn verse(n: u32) -> String {
    const ZERO_BOTTLE: &str = "no more bottles of beer";
    const WHERE: &str = "on the wall";
    const ONE_BOTTLE: &str = "1 bottle of beer";
    const BOTTLES: &str = "bottles of beer";
    const TAKE_ONE: &str = "Take one down and pass it around";
    match n {
        0 => format!("No more bottles of beer {WHERE}, \
            {ZERO_BOTTLE}.\nGo to the store and buy some more, 99 {BOTTLES} {WHERE}.\n"),
        1 => format!("{ONE_BOTTLE} {WHERE}, {ONE_BOTTLE}.\n\
            Take it down and pass it around, {ZERO_BOTTLE} {WHERE}.\n"),
        2 => format!("2 {BOTTLES} {WHERE}, 2 {BOTTLES}.\n{TAKE_ONE}, {ONE_BOTTLE} {WHERE}.\n"),
        _ => format!("{0} {BOTTLES} {WHERE}, {0} {BOTTLES}.\n{TAKE_ONE}, {1} {BOTTLES} {WHERE}.\n",
             n.to_string(), (n-1).to_string())
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();
    for n in (end..=start).rev() {
        s.push_str(&verse(n));
        if n == end {
            break;
        }
        s.push_str("\n");
    }
    s
}
