use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreEntry {
    pub name: String,
    pub score: u32,
}

#[allow(unsafe_code)]
#[cfg(target_arch = "wasm32")]
extern "C" {
    fn js_load_leaderboard(ptr: *mut u8, max_len: u32) -> u32;
    fn js_save_leaderboard(ptr: *const u8, len: u32);
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn js_load_leaderboard(_ptr: *mut u8, _max_len: u32) -> u32 { 0 }
#[cfg(not(target_arch = "wasm32"))]
unsafe fn js_save_leaderboard(_ptr: *const u8, _len: u32) { }

#[allow(unsafe_code)]
pub fn load_scores() -> Vec<ScoreEntry> {
    let mut buffer = [0u8; 4096];
    let len = unsafe { js_load_leaderboard(buffer.as_mut_ptr(), buffer.len() as u32) };
    if len == 0 {
        return Vec::new();
    }
    
    let json_str = String::from_utf8_lossy(&buffer[..len as usize]);
    serde_json::from_str(&json_str).unwrap_or_else(|_| Vec::new())
}

#[allow(unsafe_code)]
pub fn save_scores(scores: &[ScoreEntry]) {
    if let Ok(json_str) = serde_json::to_string(scores) {
        unsafe { js_save_leaderboard(json_str.as_ptr(), json_str.len() as u32) };
    }
}

pub fn add_score(name: String, score: u32) {
    let mut scores = load_scores();
    scores.push(ScoreEntry { name, score });
    scores.sort_by(|a, b| b.score.cmp(&a.score));
    scores.truncate(10);
    save_scores(&scores);
}
