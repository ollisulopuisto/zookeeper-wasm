use serde::{Deserialize, Serialize};
use shared::leaderboard::{load_list, save_list};

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
    fn js_ask_name(ptr: *mut u8, max_len: u32) -> u32;
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn js_load_leaderboard(_ptr: *mut u8, _max_len: u32) -> u32 { 0 }
#[cfg(not(target_arch = "wasm32"))]
unsafe fn js_save_leaderboard(_ptr: *const u8, _len: u32) { }
#[cfg(not(target_arch = "wasm32"))]
pub unsafe fn _js_ask_name(_ptr: *mut u8, _max_len: u32) -> u32 { 0 }

#[allow(unsafe_code)]
#[cfg(target_arch = "wasm32")]
pub fn ask_name_js() -> String {
    let mut buffer = [0u8; 16];
    let len = unsafe { js_ask_name(buffer.as_mut_ptr(), buffer.len() as u32) };
    if len == 0 { return "BUB".to_string(); }
    String::from_utf8_lossy(&buffer[..len as usize]).trim().to_string()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn ask_name_js() -> String { "BUB".to_string() }

#[allow(unsafe_code)]
pub fn load_scores() -> Vec<ScoreEntry> {
    load_list::<ScoreEntry, _>(4096, |ptr, max_len| unsafe { js_load_leaderboard(ptr, max_len) })
}

#[allow(unsafe_code)]
pub fn save_scores(scores: &[ScoreEntry]) {
    save_list(scores, |ptr, len| unsafe { js_save_leaderboard(ptr, len) });
}

pub fn add_score(name: String, score: u32) {
    let mut scores = load_scores();
    scores.push(ScoreEntry { name, score });
    scores.sort_by(|a, b| b.score.cmp(&a.score));
    scores.truncate(10);
    save_scores(&scores);
}
