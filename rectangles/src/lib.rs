use std::collections::HashMap;
pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() {
        return 0;
    }
    let matrix: Vec<Vec<char>> = 
        lines.iter().map(|l| l.chars().collect()).collect();
    //println!("to vec {:?}", matrix);
    let mut go_right: HashMap<(usize, usize), Vec<((usize, usize), usize)>> = HashMap::new();
    let mut go_down: HashMap<(usize, usize), Vec<((usize, usize), usize)>> = HashMap::new();
    
    // make horizontal edges
    // from point A go right to point B with edge length
    for (r, line) in matrix.iter().enumerate() {
        let mut start: usize;
        for (c, ch) in line.iter().enumerate() {
            if *ch == '+' {
                start = c;
                let point_a = (r, c);
                let mut x = start + 1;
                while x < line.len() {
                    if matrix[r][x] == '-' {
                        x += 1;
                    }
                    else if matrix[r][x] == '+' {
                        let point_b = (r, x);
                        go_right.entry(point_a)
                            .and_modify(|v| v.push((point_b, x - start)))
                            .or_insert(vec![(point_b, x - start)]);
                        x += 1;
                    }
                    else {
                        break;
                    }
                }
            }
        }
    }
    // make vertical edges
    // from point A go down to point B with edge length
    for c in 0..matrix[0].len() {
        let mut start: usize;
        for r in 0..matrix.len() {
            if matrix[r][c] == '+' {
                start = r;
                let point_a = (r, c);
                let mut y = start + 1;
                while y < matrix.len() {
                    if matrix[y][c] == '|' {
                        y += 1;
                    }
                    else if matrix[y][c] == '+' {
                        let point_b = (y, c);
                        go_down.entry(point_a)
                            .and_modify(|v| v.push((point_b, y - start)))
                            .or_insert(vec![(point_b, y - start)]);
                        y += 1;
                    }
                    else {
                        break;
                    }
                }
            }
        }
    }
    let mut count: u32 = 0;
    for (left, right) in go_right.iter() {
        if let Some(left_down) = go_down.get(left) {
            for right_up in right.iter() {
                if let Some(right_down) = go_down.get(&right_up.0) {
                    for e1 in left_down.iter() {
                        for e2 in right_down.iter() {
                            if (*e1).1 == (*e2).1 {
                                if let Some(edge) = go_right.get(&e1.0) {
                                    for p in edge.iter() {
                                        if p.0 == e2.0 {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    count
}
