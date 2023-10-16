use std::collections::{HashMap, HashSet};
use itertools::Itertools;
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut letters: HashSet<char> = HashSet::new();
    for c in input.chars() {
        if c.is_alphabetic() {
            letters.insert(c);
        }
    }
    if letters.len() > 10 { return None }
    let mut letter_maps: HashMap<char, u8> = HashMap::new();
    let s = input.split(&[' ', '+'][..]).filter(|s| !s.is_empty()).collect_vec();
    let permu = (0..=9).permutations(letters.len());
    for p in permu.into_iter() {
        let mut it = letters.iter();
        for n in p.iter() {
            letter_maps.insert(*(it.next()).unwrap(), *n as u8);
        }
        if check_lead_zero(&s, &letter_maps) { continue; }
        if compute(&s, &letter_maps) {
            return Some(letter_maps)
        }
    }
    return None
}

fn compute(formula: &Vec<&str>, lett_maps: &HashMap<char, u8>) -> bool {
    let right = convert_to(formula.last().unwrap(), lett_maps);
    let mut left: i64 = 0;
    for (i, s) in formula.iter().enumerate() {
        if i < formula.len()-2 {
            let l1 = convert_to(s, lett_maps);
            left += l1;
            if left > right { return false }
        }
    }
    left == right
}

fn convert_to(word: &str, lett_maps: &HashMap<char, u8>) -> i64 {
    let mut s: i64 = 0;
    for (i,c) in word.chars().rev().enumerate() {
        let digit = (*lett_maps.get(&c).unwrap()) as i64;
        s = digit*10_i64.pow(i as u32) + s;
    }
    s
}

fn check_lead_zero(input: &Vec<&str>, lett_maps: &HashMap<char, u8>) -> bool {
    let mut acc: i64 = 0;
    let mut r: i64 = 0;
    for (i, s) in input.iter().enumerate() {
        if s.contains("==") { continue; }
        if *lett_maps.get(&s.chars().nth(0).unwrap()).unwrap() == 0 && s.len() > 1 {
            return true
        }
        r = *lett_maps.get(&s.chars().last().unwrap()).unwrap() as i64;
        if i < input.len()-2  {
            acc = acc + r;
        }
    }
    if r != acc % 10 { return true }
    false
}

/*
#[test]
fn test_convert_to() {
    let hm : HashMap<char, u8> = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);
    assert_eq!(1233, convert_to("ABCC", &hm));
    assert_eq!(23, convert_to("BC", &hm));
}

#[test]
fn test_compute() {
    let hm : HashMap<char, u8> = HashMap::from([('A', 3), ('B', 6), ('C', 9)]);
    let formula = vec!["A", "B", "==", "C"];
    assert_eq!(true , compute(&formula, &hm));
}

#[test]
fn test_check_lead_zero() {
    let hm : HashMap<char, u8> = HashMap::from([('A', 0), ('B', 6), ('C', 9)]);
    let input = vec!["A", "BAB", "==", "BBC"];    
    assert!(check_lead_zero(&input, &hm));
}
*/