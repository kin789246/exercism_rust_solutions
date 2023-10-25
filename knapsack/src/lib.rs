pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(_max_weight: u32, _items: &[Item]) -> u32 {
    let mut m: Vec<Vec<u32>> = vec![vec![0; _max_weight as usize + 1]; _items.len() + 1];
    for i in 0..=_items.len() {
        for j in 0..=_max_weight as usize {
            if i == 0 || j == 0 {
                m[i][j] = 0;
            }
            else if _items[i-1].weight <= j as u32 {
                m[i][j] = std::cmp::max(m[i-1][j]
                        , m[i-1][(j as u32 - _items[i-1].weight) as usize] + _items[i-1].value);
            }
            else {
                m[i][j] = m[i-1][j];
            }
        }
    }
    
    m[_items.len()][_max_weight as usize]
}
