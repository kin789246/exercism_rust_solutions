use std::collections::HashMap;

fn main() {
    let mp: HashMap<(u8,u8), (u8, u8)> = HashMap::from([
        ((1,0),(0,0)),
        ((0,1),(1,0)),
        ((2,3),(0,1)),
        ((2,0),(2,3))
    ]);
    let mut moves: Vec<(u8,u8)> = Vec::new();
    two_bucket::save_path(&mut moves, &mp, (2,0), (0,0));
    println!("{:?}", moves);
}