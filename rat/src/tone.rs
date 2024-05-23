use crate::shared::one_pole_filter::OnePoleFilter;
use std::f32::consts::TAU;

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
    let resistor = tone * 100000. + 1500.;
    let capacitor = 3.3e-9_f32;
    (TAU * resistor * capacitor).recip()
  }
}
