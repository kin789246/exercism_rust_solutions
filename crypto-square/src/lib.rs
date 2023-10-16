pub fn encrypt(input: &str) -> String {
    if input == "" { return "".to_string(); }
    let s = input.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<Vec<char>>();
    let r = (s.len() as f32).sqrt() as usize;
    let mut c = r;
    if r*r < s.len() { c = c + 1; }
    
    let s2 = s.chunks(c).collect::<Vec<_>>();
    let mut res = vec![vec![' '; r]; c];
//    println!("slen={}, r={r}, c={c} => {:?}", s.len(), s2);
    s2.iter().enumerate().for_each( |(row, arr)| {
        arr.iter().enumerate().for_each( |(col, c)| {
            res[col][row] = *c;
        });
    });

    res.iter()
        .map( |arr| arr.iter()
        .map( |c| *c).collect::<String>() )
        .collect::<Vec<String>>()
        .join(" ")
}
