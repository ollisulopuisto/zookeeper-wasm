use macroquad::audio::{load_sound_from_bytes, Sound, play_sound, stop_sound, PlaySoundParams};

pub struct AudioManager {
    pub rotate: Sound,
    pub land: Sound,
    pub match_made: Sound,
    pub clear: Sound,
    pub music: Sound,
}

impl AudioManager {
    pub async fn new() -> Self {
        Self {
            rotate: load_sound_from_bytes(&generate_rotate_wav()).await.unwrap(),
            land: load_sound_from_bytes(&generate_land_wav()).await.unwrap(),
            match_made: load_sound_from_bytes(&generate_match_wav()).await.unwrap(),
            clear: load_sound_from_bytes(&generate_clear_wav()).await.unwrap(),
            music: load_sound_from_bytes(&generate_music_wav()).await.unwrap(),
        }
    }

    pub fn play_rotate(&self) {
        play_sound(&self.rotate, PlaySoundParams { looped: false, volume: 0.3 });
    }

    pub fn play_land(&self) {
        play_sound(&self.land, PlaySoundParams { looped: false, volume: 0.3 });
    }

    pub fn play_match(&self) {
        play_sound(&self.match_made, PlaySoundParams { looped: false, volume: 0.4 });
    }

    pub fn play_clear(&self, pitch: f32) {
        play_sound(&self.clear, PlaySoundParams { looped: false, volume: 0.4 });
        // Note: Macroquad's play_sound doesn't support pitch shifting easily in this version
        // without more complex audio management, but we'll stick to basics first.
    }

    pub fn play_music(&self) {
        play_sound(&self.music, PlaySoundParams { looped: true, volume: 0.4 });
    }

    pub fn stop_music(&self) {
        stop_sound(&self.music);
    }
}

fn create_wav_header(data_size: u32, sample_rate: u32) -> Vec<u8> {
    let mut header = Vec::with_capacity(44);
    header.extend_from_slice(b"RIFF");
    header.extend_from_slice(&(data_size + 36).to_le_bytes());
    header.extend_from_slice(b"WAVE");
    header.extend_from_slice(b"fmt ");
    header.extend_from_slice(&16u32.to_le_bytes()); 
    header.extend_from_slice(&1u16.to_le_bytes());  
    header.extend_from_slice(&1u16.to_le_bytes());  
    header.extend_from_slice(&sample_rate.to_le_bytes());
    header.extend_from_slice(&(sample_rate * 2).to_le_bytes()); 
    header.extend_from_slice(&2u16.to_le_bytes());  
    header.extend_from_slice(&16u16.to_le_bytes()); 
    header.extend_from_slice(b"data");
    header.extend_from_slice(&data_size.to_le_bytes());
    header
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

fn generate_music_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let bpm = 130.0;
    let beat_duration = 60.0 / bpm;
    let sixteen_duration = beat_duration / 4.0;
    
    let total_duration = 30.0; // 30 seconds loop
    let num_samples = (sample_rate as f32 * total_duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let midi_to_freq = |m: i32| -> f32 {
        440.0 * 2.0f32.powf((m as f32 - 69.0) / 12.0)
    };

    let bassline = [36, 36, 48, 36, 36, 36, 46, 36]; // Simple techno bass
    
    let mut noise_seed = 0x12345678u32;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sixteen_idx = (t / sixteen_duration) as usize;
        let t_sixteen = t % sixteen_duration;
        let beat_idx = (t / beat_duration) as usize;
        let t_beat = t % beat_duration;
        
        // --- Bass ---
        let b_note = bassline[sixteen_idx % 8];
        let b_freq = midi_to_freq(b_note);
        let bass = if (t * b_freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.2 } else { -0.2 };
        let bass_env = (1.0 - t_sixteen / sixteen_duration).powf(2.0);
        let bass = bass * bass_env;

        // --- Kick ---
        let mut kick = 0.0;
        if t_beat < 0.15 {
            let k_freq = 150.0 * (1.0 - t_beat / 0.15).powf(2.0) + 40.0;
            kick = (t_beat * k_freq * 2.0 * std::f32::consts::PI).sin() * 0.6 * (1.0 - t_beat / 0.15);
        }

        // --- HiHat ---
        let mut hihat = 0.0;
        if sixteen_idx % 2 == 1 && t_sixteen < 0.02 {
            noise_seed = noise_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let n = ((noise_seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
            hihat = n * 0.1 * (1.0 - t_sixteen / 0.02);
        }

        // --- Synth ---
        let s_notes = [60, 63, 65, 67];
        let s_note = s_notes[(sixteen_idx / 4) % 4];
        let s_freq = midi_to_freq(s_note);
        let mut synth = 0.0;
        if sixteen_idx % 8 >= 4 {
             synth = (t * s_freq * 2.0 * std::f32::consts::PI).sin() * 0.1 * (1.0 - (t % (sixteen_duration * 4.0)) / (sixteen_duration * 4.0));
        }

        let mixed = (bass + kick + hihat + synth) * 0.5;
        samples.push((mixed.clamp(-1.0, 1.0) * 16383.0) as i16);
    }

    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}
