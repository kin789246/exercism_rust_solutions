use std::collections::HashSet;

pub fn translate(input: &str) -> String {
    let mut output: Vec<String> = Vec::new();
    for s in input.split(' ').collect::<Vec<&str>>() {
        output.push(convert_to(s))
    }
    output.join(" ")
}

fn convert_to(input: &str) -> String {
    let vowels: HashSet<&str> = HashSet::from(["a", "e", "i", "o", "u", "xr", "yt"]);
    let consonant_cluster: HashSet<&str> = HashSet::from(["ch", "qu", "th", "rh", "sch", "thr"]);
    if vowels.iter().any(|x| input.starts_with(x)) {
        return input.to_owned()+"ay"
    }
    let cc: Vec<_> = consonant_cluster.iter().filter(|x| input.starts_with(*x)).collect();
    if cc.len() > 0 {
        let n = cc.iter().max_by_key(|x| x.len()).unwrap().len();
        if &input[n..=n] == "y" {
            return input[n..].to_owned() + &input[0..n] + "ay"
        }
        return input[n..].to_owned() + &input[0..n] + "ay"
    }
    if &input[1..3] == "qu" {
        return input[3..].to_owned() + &input[0..3] + "ay"
    }
    input[1..].to_owned() + &input[0..1] + "ay"
}