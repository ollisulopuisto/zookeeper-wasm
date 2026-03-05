use macroquad::audio::{load_sound_from_bytes, Sound, play_sound, stop_sound, PlaySoundParams};

pub struct AudioManager {
    pub jet: Sound,
    pub phase: Sound,
    pub gem: Sound,
    pub fuel: Sound,
    pub portal: Sound,
    pub game_over: Sound,
}

impl AudioManager {
    pub async fn new() -> Self {
        Self {
            jet: load_sound_from_bytes(&generate_jet_wav()).await.unwrap(),
            phase: load_sound_from_bytes(&generate_phase_wav()).await.unwrap(),
            gem: load_sound_from_bytes(&generate_gem_wav()).await.unwrap(),
            fuel: load_sound_from_bytes(&generate_fuel_wav()).await.unwrap(),
            portal: load_sound_from_bytes(&generate_portal_wav()).await.unwrap(),
            game_over: load_sound_from_bytes(&generate_game_over_wav()).await.unwrap(),
        }
    }

    pub fn start_jet(&self) {
        play_sound(&self.jet, PlaySoundParams { looped: true, volume: 0.2 });
    }

    pub fn stop_jet(&self) {
        stop_sound(&self.jet);
    }

    pub fn play_phase(&self) {
        play_sound(&self.phase, PlaySoundParams { looped: false, volume: 0.4 });
    }

    pub fn play_collect(&self) {
        play_sound(&self.gem, PlaySoundParams { looped: false, volume: 0.4 });
    }

    pub fn play_fuel(&self) {
        play_sound(&self.fuel, PlaySoundParams { looped: false, volume: 0.4 });
    }

    pub fn play_portal(&self) {
        play_sound(&self.portal, PlaySoundParams { looped: false, volume: 0.5 });
    }

    pub fn play_game_over(&self) {
        play_sound(&self.game_over, PlaySoundParams { looped: false, volume: 0.5 });
    }

    pub fn play_death(&self) {
        play_sound(&self.game_over, PlaySoundParams { looped: false, volume: 0.5 });
    }

    pub fn play_music(&self) {
        // Placeholder
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

fn generate_jet_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.15;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    let mut seed = 0x1234u32;
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let noise = ((seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
        let hum = if (t * 60.0 * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.3 } else { -0.3 };
        let sample = (noise * 0.7 + hum * 0.3) * 0.2;
        samples.push((sample * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}

fn generate_phase_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.2;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 800.0 * (1.0 - t / duration); 
        let sample = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.5 } else { -0.5 };
        let amplitude = (1.0 - t / duration).powi(2);
        samples.push((sample * amplitude * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}

fn generate_gem_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.15;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = if t < 0.05 { 1200.0 } else { 1600.0 };
        let sample = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.4 } else { -0.4 };
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}

fn generate_fuel_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.2;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 400.0 + (t * 400.0);
        let sample = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.4 } else { -0.4 };
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}

fn generate_portal_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 1.0;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let notes = [440.0, 554.37, 659.25, 880.0];
        let note_index = (t * 8.0) as usize;
        let freq = notes[note_index % 4];
        let sample = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.4 { 0.4 } else { -0.4 };
        let amplitude = 0.5 * (1.0 - t / duration);
        samples.push((sample * amplitude * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}

fn generate_game_over_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 1.2;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq1 = 200.0 * (1.0 - t / duration);
        let s1 = if (t * freq1 * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.3 } else { -0.3 };
        let amplitude = (1.0 - t / duration).powi(2);
        samples.push((s1 * amplitude * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}
