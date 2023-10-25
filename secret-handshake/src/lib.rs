pub fn actions(n: u8) -> Vec<&'static str> {
    let mut ans: Vec<&str> = Vec::new();
    let actions: Vec<&str> = vec![
        "wink",
        "double blink",
        "close your eyes",
        "jump"
    ];
    format!("{n:b}").chars()
        .rev()
        .enumerate()
        .take(5)
        .for_each(|(i, c)| {
            match c {
                '1' => {
                    if i == 4 {
                        ans.reverse();
                    }
                    else {
                        ans.push(actions[i]);
                    }
                }
                _ => return
            }
        });

    ans
}
