pub fn annotate(minefield: &[&str]) -> Vec<String> {
    //unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
    if minefield.is_empty() { return vec![] }
    let adj_diag: [(i32, i32); 8] = [
                    (-1,-1), (0,-1), (1,-1), 
                    (-1, 0),         (1, 0),
                    (-1, 1), (0, 1), (1, 1)
                ];
    let c_count = minefield.first().unwrap().len() as i32;
    if c_count == 0 { return vec!["".to_string()] }
    let r_count = minefield.len() as i32;
    let mut flat_field = minefield.iter().flat_map(|row| row.chars()).collect::<Vec<_>>();
    for i in 0..flat_field.len() {
        if flat_field[i] == '*' { continue; }
        let n = adj_diag.iter()
            .filter(|d| {
                let (x, y) = ((i as i32)%c_count+d.0, (i as i32)/c_count+d.1);
                if 0 <= x && x < c_count && 0 <= y && y < r_count {
                    let idx = y*c_count + x;
                    if flat_field[idx as usize] == '*' { return true }
                }
                return false
            })
            .count();
        if n != 0 {
            flat_field[i] = char::from_digit(n as u32, 10).unwrap();
        }
    }
    flat_field.chunks(c_count as usize)
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<_>>()
}