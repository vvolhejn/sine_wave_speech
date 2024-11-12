use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NoteName {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

pub const CHROMATIC: &[NoteName] = &[
    NoteName::C,
    NoteName::CSharp,
    NoteName::D,
    NoteName::DSharp,
    NoteName::E,
    NoteName::F,
    NoteName::FSharp,
    NoteName::G,
    NoteName::GSharp,
    NoteName::A,
    NoteName::ASharp,
    NoteName::B,
];

// C major is a diatonic scale
pub const C_MAJOR: &[NoteName] = &[
    NoteName::C,
    NoteName::D,
    NoteName::E,
    NoteName::F,
    NoteName::G,
    NoteName::A,
    NoteName::B,
];

pub const C_MAJOR_PENTATONIC: &[NoteName] = &[
    NoteName::C,
    NoteName::D,
    NoteName::E,
    NoteName::G,
    NoteName::A,
];

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FrequencyQuantizationType {
    Chromatic,
    Diatonic,
    Pentatonic,
}

impl FrequencyQuantizationType {
    pub fn to_scale(&self) -> &'static [NoteName] {
        match self {
            FrequencyQuantizationType::Chromatic => CHROMATIC,
            FrequencyQuantizationType::Diatonic => C_MAJOR,
            FrequencyQuantizationType::Pentatonic => C_MAJOR_PENTATONIC,
        }
    }
}

impl NoteName {
    fn octave_4_frequency(&self) -> f32 {
        // https://mixbutton.com/mixing-articles/music-note-to-frequency-chart/
        match self {
            NoteName::C => 261.63,
            NoteName::CSharp => 277.18,
            NoteName::D => 293.66,
            NoteName::DSharp => 311.13,
            NoteName::E => 329.63,
            NoteName::F => 349.23,
            NoteName::FSharp => 369.99,
            NoteName::G => 392.00,
            NoteName::GSharp => 415.30,
            NoteName::A => 440.00,
            NoteName::ASharp => 466.16,
            NoteName::B => 493.88,
        }
    }

    fn to_string(&self) -> String {
        match self {
            NoteName::C => "C",
            NoteName::CSharp => "C#",
            NoteName::D => "D",
            NoteName::DSharp => "D#",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::FSharp => "F#",
            NoteName::G => "G",
            NoteName::GSharp => "G#",
            NoteName::A => "A",
            NoteName::ASharp => "A#",
            NoteName::B => "B",
        }
        .to_string()
    }
}

pub fn generate_scale(
    note_names: &Vec<NoteName>,
    start_octave: i32,
    end_octave: i32,
    // Useful if you are working with normalized frequencies (radian/sample)
    // rather than Hz
    frequency_multiplier: Option<f32>,
) -> Vec<f32> {
    let frequency_multiplier = frequency_multiplier.unwrap_or(1.0);
    let mut scale = Vec::new();
    for octave in start_octave..=end_octave {
        for note in note_names {
            scale.push(note.octave_4_frequency() * 2.0f32.powi(octave - 4) * frequency_multiplier);
        }
    }
    scale.sort_by(|a, b| a.partial_cmp(b).unwrap());
    scale
}

/// Snaps a frequency to the closest note in the allowed frequencies.
pub fn quantize_frequency(to_snap: f32, allowed_frequencies: &Vec<f32>) -> f32 {
    let mut min_diff_cents = std::f32::MAX;
    let mut closest_note = 0.0;
    for note in allowed_frequencies {
        let diff_cents = (1200.0 * (to_snap / note).log2()).abs();
        if diff_cents < min_diff_cents {
            min_diff_cents = diff_cents;
            closest_note = *note;
        }
    }
    closest_note
}

pub fn quantize_frequencies(
    frequencies: &Vec<f32>,
    quantization_type: Option<FrequencyQuantizationType>,
    sample_rate: usize,
) -> Vec<f32> {
    match quantization_type {
        None => return frequencies.clone(),
        Some(quantization_type) => {
            let allowed_notes = quantization_type.to_scale();

            const MIN_OCTAVE: i32 = 0;
            const MAX_OCTAVE: i32 = 8;
            let frequency_multiplier: f32 = (2. * std::f32::consts::PI) / sample_rate as f32;
            let allowed_frequencies = generate_scale(
                &allowed_notes.to_vec(),
                MIN_OCTAVE,
                MAX_OCTAVE,
                Some(frequency_multiplier),
            );

            let quantized_frequencies = frequencies
                .iter()
                .map(|x| quantize_frequency(*x, &allowed_frequencies))
                .collect();
            quantized_frequencies
        }
    }
}

pub fn add_depth(frequencies: &Vec<f32>, width: f32) -> Vec<f32> {
    if frequencies.len() <= 1 {
        return frequencies.clone();
    }
    frequencies
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let fraction = i as f32 / (frequencies.len() - 1) as f32;
            let cur_width = width * (1. - fraction);
            // Lower by cur_width octaves
            let coef = 0.5f32.powf(cur_width);
            f * coef
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_scale() {
        let scale: Vec<f32> =
            generate_scale(&vec![NoteName::C, NoteName::E, NoteName::G], 2, 3, None)
                .iter()
                .map(|x| (x * 100.).round() / 100.)
                .collect();

        assert_eq!(scale.len(), 6);
        assert_eq!(scale[0], 65.41);
        assert_eq!(scale[1], 82.41);
        assert_eq!(scale[2], 98.00);
        assert_eq!(scale[3], 130.82);
        assert_eq!(scale[4], 164.82);
        assert_eq!(scale[5], 196.00);
    }

    #[test]
    fn test_quantize_frequency() {
        let notes = vec![100., 200., 300.];

        assert_eq!(quantize_frequency(50., &notes), 100.0);
        assert_eq!(quantize_frequency(110., &notes), 100.0);

        // We don't snap linearly but logarithmically, the breaking
        // point should be at sqrt(2) * 100 = 141.42
        assert_eq!(quantize_frequency(145.0, &notes), 200.0);

        assert_eq!(quantize_frequency(210.0, &notes), 200.0);
        assert_eq!(quantize_frequency(250.0, &notes), 300.0);
        assert_eq!(quantize_frequency(350.0, &notes), 300.0);
    }
}
