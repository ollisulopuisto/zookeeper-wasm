use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreEntry {
    pub name: String,
    pub score: u32,
}

pub fn load_scores() -> Vec<ScoreEntry> {
    shared::leaderboard::load_scores()
}

pub fn save_scores(scores: &[ScoreEntry]) {
    shared::leaderboard::save_scores(scores);
}

pub fn add_score(name: String, score: u32) {
    let mut scores = load_scores();
    scores.push(ScoreEntry { name, score });
    scores.sort_by(|a, b| b.score.cmp(&a.score));
    scores.truncate(10);
    save_scores(&scores);
}
