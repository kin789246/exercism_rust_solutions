#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        //unimplemented!("Construct a HighScores struct, given the scores: {scores:?}")
        Self { scores: scores }
    }

    pub fn scores(&self) -> &[u32] {
        //unimplemented!("Return all the scores as a slice")
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        //unimplemented!("Return the latest (last) score")
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        //unimplemented!("Return the highest score")
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        //unimplemented!("Return 3 highest scores")
        let mut v = self.scores.to_vec();
        v.sort_by(|a,b| b.cmp(a));
        v.truncate(3);
        v
    }
}
