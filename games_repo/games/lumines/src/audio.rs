use macroquad::audio::{load_sound_from_bytes, play_sound, stop_sound, PlaySoundParams, Sound};
use shared::audio::{create_wav_header, generate_music_wav};

pub struct AudioManager {
    pub rotate: Sound,
    pub land: Sound,
    pub match_made: Sound,
    pub clear: Sound,
    pub music_tracks: Vec<Sound>,
    pub current_track_idx: usize,
    pub track_durations: Vec<f32>,
    pub pending_track_idx: Option<usize>,
    pub track_time: f32,
    music_playing: bool,
    muted: bool,
}

impl AudioManager {
    pub async fn new(bpms: &[f32]) -> Self {
        let seed = macroquad::rand::gen_range(0, 0x7FFFFFFF);
        let mut music_tracks = Vec::new();
        let mut track_durations = Vec::new();

        for i in 0..bpms.len() {
            let current_bpm = bpms[i];
            let next_bpm = if i + 1 < bpms.len() {
                Some(bpms[i + 1])
            } else {
                None
            };

            let (wav, duration) = generate_music_wav(Some(seed), current_bpm, next_bpm);
            track_durations.push(duration);
            music_tracks.push(load_sound_from_bytes(&wav).await.unwrap());
        }

        Self {
            rotate: load_sound_from_bytes(&generate_rotate_wav()).await.unwrap(),
            land: load_sound_from_bytes(&generate_land_wav()).await.unwrap(),
            match_made: load_sound_from_bytes(&generate_match_wav()).await.unwrap(),
            clear: load_sound_from_bytes(&generate_clear_wav()).await.unwrap(),
            music_tracks,
            current_track_idx: 0,
            track_durations,
            pending_track_idx: None,
            track_time: 0.0,
            music_playing: false,
            muted: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.muted || !self.music_playing || self.music_tracks.is_empty() {
            return;
        }

        let loop_dur = self.track_durations[self.current_track_idx];

        self.track_time += dt;

        if self.track_time >= loop_dur {
            self.track_time = 0.0;
            if let Some(next_idx) = self.pending_track_idx {
                self.stop_music();
                self.current_track_idx = next_idx;
                self.pending_track_idx = None;
                self.play_music();
            }
        }
    }

    pub fn play_rotate(&self) {
        if !self.muted {
            play_sound(
                &self.rotate,
                PlaySoundParams {
                    looped: false,
                    volume: 0.3,
                },
            );
        }
    }

    pub fn play_land(&self) {
        if !self.muted {
            play_sound(
                &self.land,
                PlaySoundParams {
                    looped: false,
                    volume: 0.3,
                },
            );
        }
    }

    pub fn play_match(&self) {
        if !self.muted {
            play_sound(
                &self.match_made,
                PlaySoundParams {
                    looped: false,
                    volume: 0.4,
                },
            );
        }
    }

    pub fn play_clear(&self, _pitch: f32) {
        if !self.muted {
            play_sound(
                &self.clear,
                PlaySoundParams {
                    looped: false,
                    volume: 0.4,
                },
            );
        }
    }

    pub fn play_music(&mut self) {
        if !self.muted && !self.music_tracks.is_empty() {
            self.music_playing = true;
            play_sound(
                &self.music_tracks[self.current_track_idx],
                PlaySoundParams {
                    looped: true,
                    volume: 0.4,
                },
            );
        }
    }

    pub fn stop_music(&mut self) {
        if !self.music_tracks.is_empty() {
            self.music_playing = false;
            stop_sound(&self.music_tracks[self.current_track_idx]);
        }
    }

    pub fn set_track(&mut self, idx: usize) {
        if idx < self.music_tracks.len() && idx != self.current_track_idx {
            // Instead of immediate switch, we schedule it for the end of the loop.
            self.pending_track_idx = Some(idx);
        }
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
        if self.muted {
            self.stop_music();
        }
    }

    pub fn is_muted(&self) -> bool {
        self.muted
    }
}

fn generate_rotate_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.1;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 400.0 + 200.0 * (t * 50.0).sin();
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 10000.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}

fn generate_land_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.05;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 100.0 * (1.0 - t / duration);
        let sample = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.0 {
            0.5
        } else {
            -0.5
        };
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 8000.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}

fn generate_match_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.2;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 880.0;
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin() * 0.5
            + (t * freq * 1.5 * 2.0 * std::f32::consts::PI).sin() * 0.5;
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 12000.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}

fn generate_clear_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.15;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 1200.0 + 400.0 * t / duration;
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 10000.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}
