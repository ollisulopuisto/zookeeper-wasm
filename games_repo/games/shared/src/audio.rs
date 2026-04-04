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

pub fn generate_music_wav(seed: Option<u32>) -> Vec<u8> {
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

    // Arrangement tables: each entry covers one 4-bar block (8 blocks total = 32 bars).
    let mut drum_var:  [u8; 8]   = [0, 1, 2, 0, 1, 2, 1, 0];
    let mut lead_var:  [u8; 8]   = [0, 0, 0, 1, 1, 0, 0, 1];
    let mut cp_active: [bool; 8] = [false, true, false, true, true, false, false, true];

    // Shuffle the arrangements if a seed is provided
    if let Some(s) = seed {
        let mut rng = s;
        let mut next_rng = || {
            rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
            (rng >> 16) & 0x7FFF
        };

        // Fisher-Yates shuffle
        for i in (1..8).rev() {
            let j = (next_rng() % (i as u32 + 1)) as usize;
            drum_var.swap(i, j);
            let j = (next_rng() % (i as u32 + 1)) as usize;
            lead_var.swap(i, j);
            let j = (next_rng() % (i as u32 + 1)) as usize;
            cp_active.swap(i, j);
        }
    }

    let mut samples = Vec::with_capacity(num_samples);
    let mut noise_seed = 0x12345678u32;

    let bassline: [i32; 8] = [36, 36, 48, 36, 36, 36, 46, 36];
    let s_notes: [i32; 8] = [60, 63, 65, 67, 70, 72, 75, 77];
    let note_dur = sixteen_duration * 8.0;

    // Phrase variations for the sawtooth lead
    let s_step_table_2 = [0usize, 2, 4, 2, 0, 3, 5, 3];
    let s_step_table_3 = [7usize, 6, 5, 4, 3, 2, 1, 0];

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sixteen_idx = (t / sixteen_duration) as usize;
        let t_sixteen = t % sixteen_duration;
        let beat_idx = (t / beat_duration) as usize;
        let t_beat = t % beat_duration;
        let bar_idx = beat_idx / 4;
        // Position within the bar as a sixteenth-note index (0–15)
        let in_bar_16 = sixteen_idx % 16;

        // Key changes every 8 bars (4 sections) – kept coupled to section boundaries.
        let section_idx = (bar_idx / 8) % 4;
        let key_offset: i32 = match section_idx {
            0 =>  0,
            1 =>  3,
            2 =>  5,
            3 => -2,
            _ =>  0,
        };

        // Per-layer variants from the arrangement tables above.
        let block_idx = (bar_idx / 4) % 8;
        let d_var = drum_var[block_idx];
        let l_var = lead_var[block_idx];
        let cp_on = cp_active[block_idx];

        // --- Bassline ---
        let b_note = bassline[sixteen_idx % 8] + key_offset;
        let b_freq = midi_to_freq(b_note);
        let bass = if (t * b_freq * 2.0 * std::f32::consts::PI).sin() > 0.0 { 0.25f32 } else { -0.25 };
        let bass_env = (1.0 - t_sixteen / sixteen_duration).powf(1.5);
        let bass = bass * bass_env;

        // --- Kick drum ---
        let mut kick = 0.0f32;
        if d_var == 2 {
            if beat_idx % 2 == 0 && t_beat < 0.18 {
                let kf = 160.0 * (1.0 - t_beat / 0.18).powf(2.5) + 45.0;
                kick = (t_beat * kf * 2.0 * std::f32::consts::PI).sin() * 0.7 * (1.0 - t_beat / 0.18);
            }
        } else {
            if t_beat < 0.18 {
                let kf = 160.0 * (1.0 - t_beat / 0.18).powf(2.5) + 45.0;
                kick = (t_beat * kf * 2.0 * std::f32::consts::PI).sin() * 0.7 * (1.0 - t_beat / 0.18);
            }
            if d_var == 1 && matches!(in_bar_16, 2 | 10) && t_sixteen < 0.1 {
                let kf = 160.0 * (1.0 - t_sixteen / 0.1).powf(2.5) + 45.0;
                kick += (t_sixteen * kf * 2.0 * std::f32::consts::PI).sin() * 0.4 * (1.0 - t_sixteen / 0.1);
            }
        }

        // --- Hi-hat ---
        let mut hihat = 0.0f32;
        let hat_trigger = if d_var == 1 {
            true
        } else if bar_idx % 4 == 3 {
            sixteen_idx % 2 == 1 || sixteen_idx % 4 == 2
        } else {
            sixteen_idx % 2 == 1
        };
        if hat_trigger && t_sixteen < 0.03 {
            noise_seed = noise_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let n = ((noise_seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
            hihat = n * 0.12 * (1.0 - t_sixteen / 0.03);
        }

        // --- Snare ---
        let mut snare = 0.0f32;
        let is_fill_bar = d_var == 2 && bar_idx % 4 == 3;
        if is_fill_bar && in_bar_16 >= 8 && t_sixteen < 0.05 {
            noise_seed = noise_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let n = ((noise_seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
            let vol = 0.12 + 0.08 * (in_bar_16 - 8) as f32 / 7.0;
            snare = n * vol * (1.0 - t_sixteen / 0.05).powf(1.5);
        } else if d_var == 1 {
            if (beat_idx % 4 == 1 || beat_idx % 4 == 3) && t_beat < 0.1 {
                noise_seed = noise_seed.wrapping_mul(1103515245).wrapping_add(12345);
                let n = ((noise_seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
                snare = n * 0.2 * (1.0 - t_beat / 0.1).powf(1.5);
            } else if in_bar_16 == 10 && t_sixteen < 0.06 {
                noise_seed = noise_seed.wrapping_mul(1103515245).wrapping_add(12345);
                let n = ((noise_seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
                snare = n * 0.08 * (1.0 - t_sixteen / 0.06).powf(1.5);
            }
        } else if (beat_idx % 4 == 1 || beat_idx % 4 == 3) && t_beat < 0.1 {
            noise_seed = noise_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let n = ((noise_seed >> 16) as f32 / 65535.0) * 2.0 - 1.0;
            snare = n * 0.2 * (1.0 - t_beat / 0.1).powf(1.5);
        }

        // --- Lead synth and counterpoint melody ---
        let (synth, counter) = if l_var == 1 {
            let ht_step = (sixteen_idx / 8) % 8;
            let ht_freq = midi_to_freq(s_notes[ht_step] + key_offset);
            let t_in_note = t % note_dur;
            let env = if t_in_note < 0.02 {
                t_in_note / 0.02
            } else if t_in_note > note_dur - 0.04 {
                ((note_dur - t_in_note) / 0.04).max(0.0)
            } else {
                1.0
            };
            ((t * ht_freq * 2.0 * std::f32::consts::PI).sin() * 0.18 * env, 0.0f32)
        } else {
            let phrase_idx = (sixteen_idx / 16) % 4;
            let s_step: usize = match phrase_idx {
                0 => sixteen_idx % 8,
                1 => (sixteen_idx * 3) % 8,
                2 => s_step_table_2[sixteen_idx % 8],
                _ => s_step_table_3[sixteen_idx % 8],
            };
            let s_freq = midi_to_freq(s_notes[s_step] + key_offset);
            let saw = (t * s_freq % 1.0) * 2.0 - 1.0;
            let gate = if (sixteen_idx % 4 == 0) || (phrase_idx > 1 && sixteen_idx % 2 == 0) {
                (1.0 - t_sixteen / sixteen_duration).powf(0.5)
            } else {
                0.0
            };
            let s = saw * 0.15 * gate;

            let c = if cp_on {
                let cp_step = 7 - s_step; 
                let cp_freq = midi_to_freq(s_notes[cp_step] + key_offset);
                let tri_phase = t * cp_freq % 1.0;
                let tri = if tri_phase < 0.5 {
                    tri_phase * 4.0 - 1.0
                } else {
                    3.0 - tri_phase * 4.0
                };
                tri * 0.10 * gate
            } else {
                0.0
            };
            (s, c)
        };

        let mixed = (bass + kick + hihat + snare + synth + counter) * 0.5;
        samples.push((mixed.clamp(-1.0, 1.0) * 16383.0) as i16);
    }

    let mut wav = create_wav_header((num_samples * 2) as u32, sample_rate);
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}
