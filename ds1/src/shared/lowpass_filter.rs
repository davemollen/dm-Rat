use std::f32::consts::TAU;

pub struct LowpassFilter {
  sample_rate: f32,
  z: f32
}

impl LowpassFilter {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      z: 0.
    }
  }

  pub fn process(&mut self, input: f32, frequency: f32) -> f32 {
    let double_sample_rate = 2. * self.sample_rate;
	  let sample_period = double_sample_rate.recip();
	  let radians = (frequency * TAU * sample_period).tan() * double_sample_rate;

	  let b0 = radians + double_sample_rate;
	  let b1 = (radians - double_sample_rate) / b0;
	  let a0 = radians / b0;
	  let a1 = radians / b0;
    
	  let y = input * a0 + self.z;
	  self.z = input * a1 - b1 * y;
    
	  return y;
  }
}