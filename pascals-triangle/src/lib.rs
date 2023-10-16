pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut tri: Vec<Vec<u32>> = Vec::new();
        for n in 0..row_count {
            let mut nth_row: Vec<u32> = Vec::new();
            for k in 0..=n {
                nth_row.push(Self::formula1(n, k));
            }
            tri.push(nth_row);
        }
        Self { triangle: tri }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }

    fn formula1(n: u32, k: u32) -> u32 {
        (1..=n).product::<u32>() / ((1..=k).product::<u32>()*(1..=(n-k)).product::<u32>())
    }
}
