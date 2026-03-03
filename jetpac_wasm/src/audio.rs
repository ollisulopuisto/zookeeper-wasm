use macroquad::audio::{load_sound_from_bytes, Sound, play_sound, PlaySoundParams};

pub struct AudioManager {
    pub laser: Sound,
    pub explosion: Sound,
    pub jet: Sound,
    pub pickup: Sound,
    pub win: Sound,
    pub game_over: Sound,
}

impl AudioManager {
    pub async fn new() -> Self {
        Self {
            laser: load_sound_from_bytes(&generate_laser_wav()).await.unwrap(),
            explosion: load_sound_from_bytes(&generate_explosion_wav()).await.unwrap(),
            jet: load_sound_from_bytes(&generate_jet_wav()).await.unwrap(),
            pickup: load_sound_from_bytes(&generate_pickup_wav()).await.unwrap(),
            win: load_sound_from_bytes(&generate_win_wav()).await.unwrap(),
            game_over: load_sound_from_bytes(&generate_game_over_wav()).await.unwrap(),
        }
    }

    pub fn play_laser(&self) {
        play_sound(&self.laser, PlaySoundParams { looped: false, volume: 0.3 });
    }

    pub fn play_explosion(&self) {
        play_sound(&self.explosion, PlaySoundParams { looped: false, volume: 0.5 });
    }

    pub fn play_jet(&self) {
        // Jet is usually played while jetting, maybe handled externally or with volume modulation
        play_sound(&self.jet, PlaySoundParams { looped: false, volume: 0.2 });
    }

    pub fn play_pickup(&self) {
        play_sound(&self.pickup, PlaySoundParams { looped: false, volume: 0.4 });
    }

    pub fn play_win(&self) {
        play_sound(&self.win, PlaySoundParams { looped: false, volume: 0.5 });
    }

    pub fn play_game_over(&self) {
        play_sound(&self.game_over, PlaySoundParams { looped: false, volume: 0.5 });
    }
}

fn create_wav_header(data_size: u32, sample_rate: u32) -> Vec<u8> {
    let mut header = Vec::with_capacity(44);
    header.extend_from_slice(b"RIFF");
    header.extend_from_slice(&(data_size + 36).to_le_bytes());
    header.extend_from_slice(b"WAVE");
    header.extend_from_slice(b"fmt ");
    header.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1Size
    header.extend_from_slice(&1u16.to_le_bytes());  // AudioFormat (PCM)
    header.extend_from_slice(&1u16.to_le_bytes());  // NumChannels (Mono)
    header.extend_from_slice(&sample_rate.to_le_bytes());
    header.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // ByteRate
    header.extend_from_slice(&2u16.to_le_bytes());  // BlockAlign
    header.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample
    header.extend_from_slice(b"data");
    header.extend_from_slice(&data_size.to_le_bytes());
    header
}

fn generate_laser_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.15;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 1000.0 * (1.0 - t / duration); // Pitch drop
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();
        let amplitude = 1.0 - (t / duration); // Fade out
        samples.push((sample * amplitude * 16383.0) as i16);
    }

    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}

fn generate_explosion_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.4;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let mut seed = 12345u32;
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        // Simple Xorshift random for noise
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        let noise = (seed as f32 / u32::MAX as f32) * 2.0 - 1.0;
        
        let amplitude = (1.0 - t / duration).powi(2); // Sharp decay
        samples.push((noise * amplitude * 16383.0) as i16);
    }

    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}

fn generate_jet_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.1;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let mut seed = 54321u32;
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        let noise = (seed as f32 / u32::MAX as f32) * 2.0 - 1.0;
        
        let freq = 50.0;
        let hum = (t * freq * 2.0 * std::f32::consts::PI).sin();
        
        let sample = (noise * 0.5 + hum * 0.5) * 0.3;
        samples.push((sample * 16383.0) as i16);
    }

    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}

fn generate_pickup_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.2;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 440.0 * (1.0 + t / duration); // Pitch slide up
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();
        let amplitude = 1.0 - (t / duration);
        samples.push((sample * amplitude * 16383.0) as i16);
    }

    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}

fn generate_win_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 1.0;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let note_index = (t * 8.0) as usize;
        let notes = [440.0, 554.37, 659.25, 880.0, 440.0, 554.37, 659.25, 880.0];
        let freq = notes[note_index % 8];
        
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();
        let amplitude = 0.5;
        samples.push((sample * amplitude * 16383.0) as i16);
    }

    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}

fn generate_game_over_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.8;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 220.0 * (1.0 - (t / duration).powi(2));
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 16383.0) as i16);
    }

    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}
