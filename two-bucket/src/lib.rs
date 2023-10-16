use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut result: BucketStats = BucketStats { 
        moves: u8::MAX, goal_bucket: Bucket::One, other_bucket: 0 
    };
    let mut m: HashSet<(u8,u8)> = HashSet::new();
    let mut mp: HashMap<(u8,u8),(u8,u8)> = HashMap::new();
    let mut q: Vec<(u8,u8)> = Vec::new();
    let mut moves: Vec<(u8,u8)> = Vec::new();
    let mut is_solvable: bool = false;
    let start: (u8,u8);
    match start_bucket {
        Bucket::One => {
            start = (capacity_1, 0);
        },
        Bucket::Two => {
            start = (0, capacity_2);
        }
    }
    q.push(start);
    while !q.is_empty() {
        if let Some(u) = q.pop() {
            if m.contains(&u) {
                continue;
            }
            m.insert(u);
            if u.0 == goal || u.1 == goal {
                is_solvable = true;
                moves.clear();
                save_path(&mut moves, &mp, u, start);
                if result.moves > moves.len() as u8 {
                    if u.0 == goal {
                        result.goal_bucket = Bucket::One;
                        result.other_bucket = u.1;
                    }
                    else {
                        result.goal_bucket = Bucket::Two;
                        result.other_bucket = u.0;
                    }
                    result.moves = moves.len() as u8;
                }
                //println!("moves {:?}", moves);
                continue;
            }
            // filling the 1st bucket fully
            let node = (capacity_1, u.1);
            if !m.contains(&node) && u.0 == 0 {
                q.push(node);
                mp.insert(node, u);
            }
            // filling the 2nd bucket fully
            let node = (u.0, capacity_2);
            if !m.contains(&node) && u.1 == 0 {
                q.push(node);
                mp.insert(node, u);
            }
            /*
                3 5
                a to b
                (1 3) a=1 <  b.rest=2 -> (0, a+b) -> (0, 4)
                (3 4) a=3 >  b.rest=1 -> (a-b.rest, b.cap) -> (2, 5)
                (3 2) a=3 == b.rest=3 -> (a-b.rest, b.cap) -> (0, 5)

                b to a
                (1 3) b=3 > a.rest=2 -> (a.cap, b-a.rest) -> (3, 1)
                (2 1) b=1 == a.rest=1 -> (a.cap, b-a.rest) -> (3, 0)
                (1 1) b=1 < a.rest=2 -> (a+b, 0) -> (2, 0)

                5 3
                a to b
                (1 2) a=1 == b.rest=1 -> (a-b.rest, b.cap) -> (0, 3)
                (2 2) a=2 > b.rest=1 -> (a-b.rest, b.cap) -> (1, 3)
                (1 1) a=1 < b.rest=2 -> (0, a+b) -> (0, 2)
                
                b to a
                (5 2) b=2 > a.rest=0 -> (a.cap, b-a.rest) -> (5, 2)
                (1 3) b=3 < a.rest=4 -> (a+b, 0) -> (4, 0)
                (2, 3) b=3 == a.rest=3 -> (a.cap, b-a.rest) -> (5, 0)
             */
            if *start_bucket == Bucket::One {
                // pouring water from 1st bucket to 2nd bucket
                let rest_1_2 = capacity_2 - u.1;
                if rest_1_2 <= u.0 {
                    let node = (u.0-rest_1_2, capacity_2);
                    if !m.contains(&node) {
                        q.push(node);
                        mp.insert(node, u);
                    } 
                }
                else {
                    let node = (0, u.0+u.1);
                    if !m.contains(&node) {
                        q.push(node);
                        mp.insert(node, u);
                    }
                }
            }
            else {
                // pouring water from 2nd bucket to 1st bucket
                let rest_2_1 = capacity_1 - u.0;
                if rest_2_1 <= u.1 {
                    let node = (capacity_1, u.1-rest_2_1);
                    if !m.contains(&node) {
                        q.push(node);
                        mp.insert(node, u);
                    } 
                }
                else {
                    let node = (u.0+u.1, 0);
                    if !m.contains(&node) {
                        q.push(node);
                        mp.insert(node, u);
                    }
                }
            }
            // empty 1st bucket
            let node = (0, u.1);
            if !m.contains(&node) {
                q.push(node);
                mp.insert(node, u);
            }
            // empty 2nd bucket
            let node = (u.0, 0);
            if !m.contains(&node) {
                q.push(node);
                mp.insert(node, u);
            }
        }
    }
    if is_solvable {
        Some(result)
    }
    else {
        None
    }
}

pub fn save_path(moves:&mut Vec<(u8,u8)>, mp: &HashMap<(u8,u8),(u8,u8)>, 
    mv:(u8,u8), start:(u8,u8)) {
    if mv == start {
        (*moves).push(start);
        return;
    }
    save_path(moves, mp, *(*mp).get(&mv).unwrap(), start);
    (*moves).push(mv);
}