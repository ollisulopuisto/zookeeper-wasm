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
    let bpm = 120.0;
    let beat_duration = 60.0 / bpm;
    let sixteen_duration = beat_duration / 4.0;
    
    // C Major Scale: 2 Octaves
    let scale = [
        261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, // C4-B4
        523.25, 587.33, 659.25, 698.46, 783.99, 880.00, 987.77, // C5-B5
        1046.50 // C6
    ];
    
    let mut seed = 123u32;
    let mut next_rand = |max: usize| -> usize {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((seed >> 16) as usize) % max
    };

    let mut song_melody = Vec::new();
    let mut song_bass = Vec::new();
    
    // Generate 10 distinct sections
    for _ in 0..10 {
        let root_idx = next_rand(7); // Random starting note in the first octave
        let mut current_idx = root_idx + 7; // Start in middle octave
        
        for i in 0..16 {
            // Random walk: Move current index by -2, -1, 0, 1, or 2
            let step = (next_rand(5) as i32) - 2;
            current_idx = (current_idx as i32 + step).clamp(0, (scale.len() - 1) as i32) as usize;
            
            let note = if i % 4 == 0 { scale[root_idx + 7] } // Strong root on downbeat
                      else if i % 2 == 0 { scale[current_idx] }
                      else if next_rand(3) == 0 { 0.0 } // Occasional rest
                      else { scale[current_idx] };
            
            song_melody.push(note);
            
            // Bass: Simple root note matching the melody section's key
            let bass_note = scale[root_idx] / 2.0;
            song_bass.push(if i % 8 < 4 { bass_note } else { bass_note * 0.75 }); // Simple 1-5 bass
        }
    }
    
    let num_sixteens = song_melody.len();
    let total_duration = sixteen_duration * num_sixteens as f32;
    let num_samples = (sample_rate as f32 * total_duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let mut noise_seed = 0x1337u32;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let idx = (t / sixteen_duration) as usize % song_melody.len();
        let t_beat = t % beat_duration;
        let beat_idx = (t / beat_duration) as usize;
        
        // --- Lead Channel (Triangle-like pulse) ---
        let mut lead = 0.0;
        if song_melody[idx] > 0.0 {
            let freq = song_melody[idx];
            lead = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.1 } else { -0.1 };
            lead *= 1.0 - (t % sixteen_duration) / sixteen_duration; // Decay
        }
        
        // --- Bass Channel (Soft square) ---
        let bass_freq = song_bass[idx];
        let mut bass = if (t * bass_freq * 2.0 * std::f32::consts::PI).sin() > 0.5 { 0.15 } else { -0.15 };
        bass *= 0.8 + 0.2 * (t * 2.0 * std::f32::consts::PI).cos(); // Slight volume swell

        // --- Drum Channel ---
        let mut drums = 0.0;
        // Kick
        if t_beat < 0.07 {
            let k_freq = 60.0 * (1.0 - t_beat / 0.07) + 35.0;
            drums += (t * k_freq * 2.0 * std::f32::consts::PI).sin() * 0.2;
        }
        // Snare
        if beat_idx % 2 == 1 && t_beat < 0.07 {
            noise_seed = noise_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let n = ((noise_seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
            drums += n * (1.0 - t_beat / 0.07) * 0.15;
        }

        let mixed = (lead + bass + drums) * 0.7;
        samples.push((mixed * 16383.0) as i16);
    }

    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    wav
}
