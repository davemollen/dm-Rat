use crate::shared::one_pole_filter::OnePoleFilter;
use std::f32::consts::TAU;

const R1: f32 = 100000.;
const R2: f32 = 1500.;
const C1: f32 = 3.3e-9;

pub struct Tone {
  lowpass: OnePoleFilter,
  prev_tone: Option<f32>,
  freq: f32,
}

impl Tone {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass: OnePoleFilter::new(sample_rate),
      prev_tone: None,
      freq: 0.,
    }
  }

  pub fn reset(&mut self) {
    self.lowpass.reset();
  }

  pub fn process(&mut self, input: f32, tone: f32) -> f32 {
    if self.prev_tone.map_or(true, |value| value != tone) {
      self.freq = self.get_cutoff_frequency(tone);
      self.prev_tone = Some(tone);
    }
    self.lowpass.process(input, self.freq)
  }

  fn get_cutoff_frequency(&self, tone: f32) -> f32 {
    let resistor = tone * R1 + R2;
    (TAU * resistor * C1).recip()
  }
}
