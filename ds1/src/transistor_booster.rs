use std::f32::consts::TAU;

pub struct TransistorBooster {
  z1: f32,
  z2: f32,
  sample_rate: f32
}

impl TransistorBooster {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      z1: 0.,
      z2: 0.,
      sample_rate
    }
  }

  pub fn process(&mut self, input: f32, gain: f32) -> f32 { 
    let double_sample_rate = 2. * self.sample_rate;
    let sample_period = double_sample_rate.recip();
    
    let freq1 = 3.077643;
    let freq2 = 703.162476;
    
    let radians1 = (freq1 * TAU * sample_period).tan() * double_sample_rate;
    let radians2 = (freq2 * TAU * sample_period).tan() * double_sample_rate;
    
    let a = radians1 + double_sample_rate;
    let b = radians1 - double_sample_rate;
    let c = radians2 + double_sample_rate;
    let d = radians2 - double_sample_rate;
    
    let b0 = a * c;
    let b1 = (a * d + b * c) / b0;
    let b2 = b * d / b0;
    let a0 = double_sample_rate * double_sample_rate / b0 * gain;
    let a1 = a0 * -2.;
    let a2 = a0;
    
    let y = input * a0 + self.z1;
    self.z1 = input * a1 + self.z2 - b1 * y;
    self.z2 = input * a2 - b2 * y;

    y
  }
}