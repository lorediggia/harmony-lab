use rodio::Source;
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Clone)]
pub struct PianoWave {
    freq: f32,
    sample_rate: u32,
    current_sample: usize,
    duration_samples: usize,
}

impl PianoWave {
    pub fn new(freq: f32) -> Self {
        Self {
            freq,
            sample_rate: 44100,
            current_sample: 0,
            duration_samples: 44100 * 2,
        }
    }
}

impl Iterator for PianoWave {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if self.current_sample >= self.duration_samples {
            return None;
        }
        let t = self.current_sample as f32 / self.sample_rate as f32;
        let f = self.freq;

        let wave = (t * f * 2.0 * PI).sin()
            + 0.18 * (t * f * 4.0 * PI).sin()
            + 0.06 * (t * f * 6.0 * PI).sin();

        self.current_sample += 1;
        Some(wave * (-t * 3.5).exp() * 0.15)
    }
}

impl Source for PianoWave {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { Some(Duration::from_secs(2)) }
}

#[derive(Clone)]
pub struct GuitarWave {
    freq: f32,
    sample_rate: u32,
    current_sample: usize,
    duration_samples: usize,
}

impl GuitarWave {
    pub fn new(freq: f32) -> Self {
        Self {
            freq,
            sample_rate: 44100,
            current_sample: 0,
            duration_samples: 44100 * 3,
        }
    }
}

impl Iterator for GuitarWave {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if self.current_sample >= self.duration_samples {
            return None;
        }
        let t = self.current_sample as f32 / self.sample_rate as f32;
        let f = self.freq;

        let wave = (t * f * 2.0 * PI).sin()
            + 0.45 * (t * f * 4.0 * PI).sin()
            + 0.22 * (t * f * 6.0 * PI).sin()
            + 0.08 * (t * f * 8.0 * PI).sin();

        let env = (-t * 5.5).exp() * 0.7 + (-t * 1.2).exp() * 0.3;

        self.current_sample += 1;
        Some(wave * env * 0.1)
    }
}

impl Source for GuitarWave {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { Some(Duration::from_secs(3)) }
}