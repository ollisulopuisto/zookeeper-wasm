use macroquad::audio::{load_sound_from_bytes, Sound, play_sound, stop_sound, PlaySoundParams};
use shared::audio::{create_wav_header, generate_music_wav};

pub struct AudioManager {
    pub rotate: Sound,
    pub land: Sound,
    pub match_made: Sound,
    pub clear: Sound,
    pub music: Sound,
    muted: bool,
}

impl AudioManager {
    pub async fn new() -> Self {
        let seed = macroquad::rand::gen_range(0, 0x7FFFFFFF);
        Self {
            rotate: load_sound_from_bytes(&generate_rotate_wav()).await.unwrap(),
            land: load_sound_from_bytes(&generate_land_wav()).await.unwrap(),
            match_made: load_sound_from_bytes(&generate_match_wav()).await.unwrap(),
            clear: load_sound_from_bytes(&generate_clear_wav()).await.unwrap(),
            music: load_sound_from_bytes(&generate_music_wav(Some(seed))).await.unwrap(),
            muted: false,
        }
    }

    pub fn play_rotate(&self) {
        if !self.muted {
            play_sound(&self.rotate, PlaySoundParams { looped: false, volume: 0.3 });
        }
    }

    pub fn play_land(&self) {
        if !self.muted {
            play_sound(&self.land, PlaySoundParams { looped: false, volume: 0.3 });
        }
    }

    pub fn play_match(&self) {
        if !self.muted {
            play_sound(&self.match_made, PlaySoundParams { looped: false, volume: 0.4 });
        }
    }

    pub fn play_clear(&self, _pitch: f32) {
        if !self.muted {
            play_sound(&self.clear, PlaySoundParams { looped: false, volume: 0.4 });
        }
        // Note: Macroquad's play_sound doesn't support pitch shifting easily in this version
        // without more complex audio management, but we'll stick to basics first.
    }

    pub fn play_music(&self) {
        if !self.muted {
            play_sound(&self.music, PlaySoundParams { looped: true, volume: 0.4 });
        }
    }

    pub fn stop_music(&self) {
        stop_sound(&self.music);
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
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
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
        let sample = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.5 } else { -0.5 };
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 8000.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
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
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin() * 0.5 + (t * freq * 1.5 * 2.0 * std::f32::consts::PI).sin() * 0.5;
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 12000.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
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
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}
