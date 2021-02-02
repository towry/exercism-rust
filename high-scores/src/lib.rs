#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a[u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            scores: scores
        }
    }

    pub fn scores(&self) -> &'a [u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|&x| x)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().map(|&x| x)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut cloned = self.scores.to_vec();
        cloned.sort_by(|a, b| b.cmp(a));

        let end = if cloned.len() >= 3 { 3 } else { cloned.len() };
        cloned.get(0..end).unwrap().to_vec()
    }
}
