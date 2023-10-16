pub fn encode(n: u64) -> String {
    if n == 0 { return "zero".to_string() }
    let large_numbers = vec!["", " thousand", " million", " billion", " trillion", " quadrillion", " quintillion"];
    let mut split_n: Vec<u64> = Vec::new();
    let mut nn: u64 = n;
    let mut ans: Vec<String> = Vec::new();
    while nn > 0 {
        split_n.push(nn%1000);
        nn /= 1000;
    }
    for (i, no) in split_n.iter().enumerate() {
        if *no != 0 {
            ans.push(format!("{}{}", less_1000(*no), large_numbers[i]));
        }
    }
    ans.reverse();
    ans.join(" ")
}

pub fn less_1000(n: u64) -> String {
    let zero_to_19: Vec<&str> = vec![
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen"
    ];
    let tens: Vec<&str> = vec![
        "",
        "",
        "twenty",
        "thirty",
        "forty",
        "fifty",
        "sixty",
        "seventy",
        "eighty",
        "ninety"
    ];
    match n {
        0..=19 => return zero_to_19[n as usize].to_string(),
        20..=99 => {
            if n%10 == 0 {
                return format!("{}", tens[(n/10) as usize])
            }
            return format!("{}-{}", tens[(n/10) as usize], zero_to_19[(n%10) as usize])
        },
        _ => {
            if n%100 == 0 {
                return format!("{} hundred", zero_to_19[(n/100) as usize])
            }
            return format!("{} hundred {}", zero_to_19[(n/100) as usize], less_1000(n%100))
        } 
    }
}