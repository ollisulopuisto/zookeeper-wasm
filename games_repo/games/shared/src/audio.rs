pub fn create_wav_header(data_size: u32, sample_rate: u32) -> Vec<u8> {
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

pub fn generate_music_wav() -> Vec<u8> {
    let sample_rate = 44100;
    let bpm = 130.0;
    let beat_duration = 60.0 / bpm;
    let sixteen_duration = beat_duration / 4.0;

    let bars = 32;
    let beats_per_bar = 4;
    let total_beats = bars * beats_per_bar;
    let total_duration = total_beats as f32 * beat_duration;
    let num_samples = (sample_rate as f32 * total_duration) as usize;

    let midi_to_freq = |m: i32| -> f32 { 440.0 * 2.0f32.powf((m as f32 - 69.0) / 12.0) };

    let mut samples = Vec::with_capacity(num_samples);
    let mut noise_seed = 0x12345678u32;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sixteen_idx = (t / sixteen_duration) as usize;
        let t_sixteen = t % sixteen_duration;
        let beat_idx = (t / beat_duration) as usize;
        let t_beat = t % beat_duration;
        let bar_idx = beat_idx / 4;

        let key_offset = match (bar_idx / 8) % 4 {
            0 => 0,
            1 => 3,
            2 => 5,
            3 => -2,
            _ => 0,
        };

        let bassline = [36, 36, 48, 36, 36, 36, 46, 36];
        let b_note = bassline[sixteen_idx % 8] + key_offset;
        let b_freq = midi_to_freq(b_note);
        let bass = if (t * b_freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.25 } else { -0.25 };
        let bass_env = (1.0 - t_sixteen / sixteen_duration).powf(1.5);
        let bass = bass * bass_env;

        let mut kick = 0.0;
        if t_beat < 0.18 {
            let k_freq = 160.0 * (1.0 - t_beat / 0.18).powf(2.5) + 45.0;
            kick = (t_beat * k_freq * 2.0 * std::f32::consts::PI).sin() * 0.7 * (1.0 - t_beat / 0.18);
        }

        let mut hihat = 0.0;
        let hat_trigger = if bar_idx % 4 == 3 {
            sixteen_idx % 2 == 1 || sixteen_idx % 4 == 2
        } else {
            sixteen_idx % 2 == 1
        };
        if hat_trigger && t_sixteen < 0.03 {
            noise_seed = noise_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let n = ((noise_seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
            hihat = n * 0.12 * (1.0 - t_sixteen / 0.03);
        }

        let mut snare = 0.0;
        if (beat_idx % 4 == 1 || beat_idx % 4 == 3) && t_beat < 0.1 {
            noise_seed = noise_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let n = ((noise_seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
            snare = n * 0.2 * (1.0 - t_beat / 0.1).powf(1.5);
        }

        let s_notes = [60, 63, 65, 67, 70, 72, 75, 77];
        let phrase_idx = (sixteen_idx / 16) % 4;
        let s_step = match phrase_idx {
            0 => sixteen_idx % 8,
            1 => (sixteen_idx * 3) % 8,
            2 => [0, 2, 4, 2, 0, 3, 5, 3][sixteen_idx % 8],
            _ => [7, 6, 5, 4, 3, 2, 1, 0][sixteen_idx % 8],
        };
        let s_note = s_notes[s_step as usize] + key_offset;
        let s_freq = midi_to_freq(s_note);
        let saw = (t * s_freq % 1.0) * 2.0 - 1.0;
        let mut synth = saw * 0.15;
        let gate = if (sixteen_idx % 4 == 0) || (phrase_idx > 1 && sixteen_idx % 2 == 0) {
            (1.0 - t_sixteen / sixteen_duration).powf(0.5)
        } else {
            0.0
        };
        synth *= gate;

        let mixed = (bass + kick + hihat + snare + synth) * 0.5;
        samples.push((mixed.clamp(-1.0, 1.0) * 16383.0) as i16);
    }

    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}
