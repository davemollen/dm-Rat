use super::delay_line::DelayLine;
use super::lowpass::Lowpass;
use super::mix::Mix;
use std::f32;

pub struct Repeat {
  delay: DelayLine,
  smooth_freq: Lowpass,
}

impl Repeat {
  pub fn new(sample_rate: f64) -> Self {
    Self {
      delay: DelayLine::new(sample_rate as usize * 5, sample_rate),
      smooth_freq: Lowpass::new(sample_rate),
    }
  }

  fn repeat(&mut self, freq: f32, repeats: u32, feedback: f32) -> f32 {
    let time_interval = 1000. / freq;
    let mut out = 0f32;
    for i in 1..repeats {
      let index = i as f32;
      out += self.delay.read(
        time_interval * index,
        "step",
      ) * f32::powf(2., index - 1.0) * feedback;
      
    }
    out
  }

  pub fn run(
    &mut self,
    input: f32,
    freq: f32,
    repeats: f32,
    feedback: f32,
    mix: f32,
  ) -> f32 {
    self.delay.write(input);
    let frequency = self.smooth_freq.run(freq, 3.);
    let repeated = self.repeat(frequency, repeats as u32, feedback);
    Mix::run(input, repeated, mix)
  }
}
