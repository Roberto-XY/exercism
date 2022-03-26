#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.scores
            .iter()
            .fold([None, None, None], |high_scores, score| match high_scores {
                [first, second, _] if is_higher(score, first) => [Some(score), first, second],

                [first, second, _] if is_higher(score, second) => [first, Some(score), second],

                [first, second, third] if is_higher(score, third) => [first, second, Some(score)],

                high_scores => high_scores,
            })
            .iter()
            .flatten()
            .map(|x| **x)
            .collect()
    }
}

fn is_higher(score_a: &u32, score_b: Option<&u32>) -> bool {
    score_b.map(|x| score_a > x).unwrap_or(true)
}
