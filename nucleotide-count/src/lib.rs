use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if dna == "" { return Ok(0) }
    if dna.chars().any(|c| c!='A' && c!='T' && c!='C' && c!='G') {
        return Err('X')
    }
    let r: usize = dna.chars().filter(|c| *c == nucleotide).count();
    if r != 0 { return Ok(r) }
    else { Err('X') }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut r: HashMap<char, usize> = HashMap::from([('A', 0), ('T', 0), ('C', 0), ('G', 0)]);
    for c in dna.chars() {
        match c {
            'A' => *r.get_mut(&'A').unwrap() += 1,
            'T' => *r.get_mut(&'T').unwrap() += 1,
            'C' => *r.get_mut(&'C').unwrap() += 1,
            'G' => *r.get_mut(&'G').unwrap() += 1,
            _ => return Err('X')
        }
    }
    Ok(r)
}
