use std::{fmt::Display, collections::HashMap};
use std::cmp::Ordering;
#[derive(Debug)]
struct Team {
    name: String,
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32
}

impl Team {
    fn new(name: &str) -> Self {
        Self { name:name.to_string(), mp: 0, w: 0, d: 0, l: 0, p: 0 }
    }
    fn win(&mut self) {
        self.mp += 1;
        self.w += 1;
        self.p += 3;
    } 
    fn loss(&mut self) {
        self.mp += 1;
        self.l += 1;
    }  
    fn draw(&mut self) {
        self.mp += 1;
        self.d += 1;
        self.p += 1;
    }  
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:31}|  {} |  {} |  {} |  {} |  {}", self.name, self.mp, self.w, self.d, self.l, self.p)
    }
}

pub fn tally(match_results: &str) -> String {
    let mut ans: String = String::from("Team                           | MP |  W |  D |  L |  P");
    if match_results.len() == 0 { return ans }
    let mut teams: HashMap<&str, Team> = HashMap::new();
    for str in match_results.lines() {
        let matched: Vec<_> = str.split(';').collect();
        if !teams.contains_key(matched[0]) {
            teams.insert(matched[0], Team::new(matched[0]));
        }
        if !teams.contains_key(matched[1]) {
            teams.insert(matched[1], Team::new(matched[1]));
        }
        match matched[2] {
            "win" => {
                teams.get_mut(matched[0]).unwrap().win();
                teams.get_mut(matched[1]).unwrap().loss();
            },
            "loss" => {
                teams.get_mut(matched[0]).unwrap().loss();
                teams.get_mut(matched[1]).unwrap().win();
            },
            _ => {
                teams.get_mut(matched[0]).unwrap().draw();
                teams.get_mut(matched[1]).unwrap().draw();                
            }
        }
    }
    let mut v: Vec<_> = teams.values_mut().collect();
    v.sort_by(|a, b| {
        let point = b.p.cmp(&a.p); 
        if point == Ordering::Equal {
            return a.name.cmp(&b.name)
        }
        point
    });
    v.iter().for_each(|t| {
        ans += &("\n".to_owned() + &t.to_string());
    });
    ans
}
