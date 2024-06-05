use crate::shared::one_pole_filter::OnePoleFilter;
use std::f32::consts::TAU;

const R1: f32 = 100000.;
const R2: f32 = 1500.;
const C1: f32 = 3.3e-9;

pub struct Tone {
  lowpass: OnePoleFilter,
}

impl Tone {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass: OnePoleFilter::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, tone: f32) -> f32 {
    let cutoff_freq = self.get_cutoff_frequency(tone);
    self.lowpass.process(input, cutoff_freq)
  }

  fn get_cutoff_frequency(&self, tone: f32) -> f32 {
    let resistor = tone * R1 + R2;
    (TAU * resistor * C1).recip()
  }
}
