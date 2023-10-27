use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counter = |input: &[&str]| {
        let mut m: HashMap<char, usize> = HashMap::new();
        for line in input {
            for c in line.chars()
                    .filter(|c| c.is_alphabetic()) 
                    .map(|c| c.to_ascii_lowercase())
            {
                *m.entry(c).or_default() += 1;
            }
        }
        m
    };
    
    match input.len() {
        0 => HashMap::new(),
        n if n<500 => counter(input),
        _ => thread::scope(|s| {
            let mut handles = Vec::with_capacity(worker_count);
            for lines in input.chunks(input.len()/worker_count + 1) {
                handles.push(s.spawn(|| counter(lines)));
            }
            let mut map = handles.pop().unwrap().join().unwrap();
            for h in handles {
                h.join().unwrap()
                    .into_iter()
                    .for_each(|(k, v)| {
                        *map.entry(k).or_default() += v;
                    });
            }
            map
        })
    }
    // todo!(
    //     "Count the frequency of letters in the given input '{input:?}'. Ensure that you are using {} to process the input.",
    //     match worker_count {
    //         1 => "1 worker".to_string(),
    //         _ => format!("{worker_count} workers"),
    //     }
    // );
}
