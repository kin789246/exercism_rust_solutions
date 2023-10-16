pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    use std::iter;
    if size == 0 { return Vec::new() }
    let mut matrix1: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];
    let direction: Vec<(i32, i32)> = vec![
        (1, 0), // right
        (0, 1), // down
        (-1, 0),// left
        (0, -1),// up
    ];
    let (mut i, mut j, mut n) = (-1, 0, 1..);
    let mut movement = direction.iter().cycle();
    for (mv_i, mv_j) in iter::once(size)
        .chain((1..size).rev().flat_map(|x| iter::repeat(x).take(2)))
        .flat_map(|x| iter::repeat(movement.next().unwrap()).take(x as usize)) {
            i += mv_i;
            j += mv_j;
            matrix1[j as usize][i as usize] = n.next().unwrap();
    }   
    matrix1
}

/*
 1  2  3  4 5
16 17 18 19 6
15 24 25 20 7
14 23 22 21 8
13 12 11 10 9

r -> 5
d -> 4
l -> 4
u -> 3
r -> 3
d -> 2
l -> 2
u -> 1
r -> 1

 1  2  3  4 
12 13 14  5
11 16 15  6
10  9  8  7

r -> 4
d -> 3
l -> 3
u -> 2
r -> 2
d -> 1
l -> 1

1  2  3  4  5  6
20 21 22 23 24 7
19 32 33 34 25 8
18 31 36 35 26 9
17 30 29 28 27 10
16 15 14 13 12 11

r -> 6
d -> 5
l -> 5
u -> 4
r -> 4
d -> 3
l -> 3
u -> 2
r -> 2
d -> 1
l -> 1
 */
