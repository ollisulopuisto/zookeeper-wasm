use macroquad::audio::{load_sound_from_bytes, Sound, play_sound, stop_sound, PlaySoundParams};

pub struct AudioManager {
    pub jump: Sound,
    pub bubble_blow: Sound,
    pub bubble_pop: Sound,
    pub enemy_trapped: Sound,
    pub fruit_collect: Sound,
    pub death: Sound,
    pub music: Sound,
}

impl AudioManager {
    pub async fn new() -> Self {
        Self {
            jump: load_sound_from_bytes(&generate_jump_wav()).await.unwrap(),
            bubble_blow: load_sound_from_bytes(&generate_bubble_blow_wav()).await.unwrap(),
            bubble_pop: load_sound_from_bytes(&generate_bubble_pop_wav()).await.unwrap(),
            enemy_trapped: load_sound_from_bytes(&generate_enemy_trapped_wav()).await.unwrap(),
            fruit_collect: load_sound_from_bytes(&generate_fruit_collect_wav()).await.unwrap(),
            death: load_sound_from_bytes(&generate_death_wav()).await.unwrap(),
            music: load_sound_from_bytes(&generate_music_wav()).await.unwrap(),
        }
    }

    pub fn play_jump(&self) {
        play_sound(&self.jump, PlaySoundParams { looped: false, volume: 0.3 });
    }

    pub fn play_bubble_blow(&self) {
        play_sound(&self.bubble_blow, PlaySoundParams { looped: false, volume: 0.3 });
    }

    pub fn play_bubble_pop(&self) {
        play_sound(&self.bubble_pop, PlaySoundParams { looped: false, volume: 0.3 });
    }

    pub fn play_enemy_trapped(&self) {
        play_sound(&self.enemy_trapped, PlaySoundParams { looped: false, volume: 0.4 });
    }

    pub fn play_fruit_collect(&self) {
        play_sound(&self.fruit_collect, PlaySoundParams { looped: false, volume: 0.4 });
    }

    pub fn play_death(&self) {
        play_sound(&self.death, PlaySoundParams { looped: false, volume: 0.5 });
    }

    pub fn play_music(&self) {
        play_sound(&self.music, PlaySoundParams { looped: true, volume: 0.25 });
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

fn generate_jump_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.15;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 200.0 + 800.0 * t / duration;
        let sample = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.4 } else { -0.4 };
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}

fn generate_bubble_blow_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.1;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 1200.0 * (1.0 - t / duration);
        let sample = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.3 } else { -0.3 };
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}

fn generate_bubble_pop_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.05;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    let mut seed = 0x5678u32;
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let noise = ((seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
        let amplitude = (1.0 - t / duration).powi(2);
        samples.push((noise * amplitude * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}

fn generate_enemy_trapped_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.3;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = if (t * 10.0) as i32 % 2 == 0 { 800.0 } else { 1000.0 };
        let sample = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.4 } else { -0.4 };
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}

fn generate_fruit_collect_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.2;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = if t < 0.1 { 1000.0 } else { 1500.0 };
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();
        let amplitude = 1.0 - t / duration;
        samples.push((sample * amplitude * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}

fn generate_death_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let duration = 0.8;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 400.0 * (1.0 - t / duration);
        let sample = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.5 { 0.5 } else { -0.5 };
        let amplitude = (1.0 - t / duration).powi(2);
        samples.push((sample * amplitude * 16383.0) as i16);
    }
    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}

fn generate_music_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let bpm = 125.0;
    let beat_duration = 60.0 / bpm;
    let sixteen_duration = beat_duration / 4.0;
    
    // Scale: C Major / A Minor
    let scale = [261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25];
    
    let mut seed = 42u32;
    let mut next_rand = |max: usize| -> usize {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((seed >> 16) as usize) % max
    };

    // Generate 10 different "song sections"
    let mut song_melody = Vec::new();
    let mut song_bass = Vec::new();
    
    for _ in 0..10 {
        let mut section_melody = Vec::new();
        let mut section_bass = Vec::new();
        let root_idx = next_rand(scale.len());
        
        for i in 0..16 {
            // Semi-random melody based on scale
            let note = if i % 4 == 0 { scale[root_idx] }
                      else if i % 2 == 0 { scale[next_rand(scale.len())] }
                      else { scale[next_rand(scale.len())] * (if next_rand(2) == 0 { 1.0 } else { 2.0 }) };
            section_melody.push(note);
            
            // Bass matches the root or follows simply
            section_bass.push(scale[root_idx] / 2.0);
        }
        song_melody.extend(section_melody);
        song_bass.extend(section_bass);
    }
    
    let num_sixteens = song_melody.len();
    let total_duration = sixteen_duration * num_sixteens as f32;
    let num_samples = (sample_rate as f32 * total_duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let mut noise_seed = 0x9876u32;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sixteen_idx = (t / sixteen_duration) as usize % song_melody.len();
        let beat_idx = (t / beat_duration) as usize;
        
        // Lead
        let freq = song_melody[sixteen_idx];
        let lead = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.12 } else { -0.12 };
        let lead_env = 1.0 - (t % sixteen_duration) / sixteen_duration;
        
        // Bass
        let bass_freq = song_bass[sixteen_idx];
        let bass = if (t * bass_freq * 2.0 * std::f32::consts::PI).sin() > 0.4 { 0.18 } else { -0.18 };
        let bass_env = if sixteen_idx % 2 == 0 { 1.0 } else { 0.7 };

        // Drums
        let mut drums = 0.0;
        let t_beat = t % beat_duration;
        
        // Kick
        if t_beat < 0.08 {
            let kick_freq = 50.0 * (1.0 - t_beat / 0.08) + 30.0;
            drums += (t * kick_freq * 2.0 * std::f32::consts::PI).sin() * 0.25;
        }

        // Snare
        if beat_idx % 2 == 1 && t_beat < 0.08 {
            noise_seed = noise_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let noise = ((noise_seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
            drums += noise * (1.0 - t_beat / 0.08) * 0.2;
        }

        let mixed = (lead * lead_env + bass * bass_env + drums) * 0.7;
        samples.push((mixed * 16383.0) as i16);
    }

    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}
