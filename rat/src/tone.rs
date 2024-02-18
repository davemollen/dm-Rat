use crate::shared::lowpass_filter::LowpassFilter;
use std::f32::consts::TAU;

pub struct Tone {
  lowpass: LowpassFilter,
}

impl Tone {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass: LowpassFilter::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, tone: f32) -> f32 {
    let resistor = tone * 100000. + 1500.;
    let capacitor = 3.3e-9_f32;
    let radians_per_sec = (resistor * capacitor).recip();
    let frequency = radians_per_sec / TAU;
    self.lowpass.process(input, frequency)
  }
}
