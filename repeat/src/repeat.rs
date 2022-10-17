use super::delay_line::DelayLine;
use super::lowpass::Lowpass;
use std::f32;

pub struct Repeat {
  delay: DelayLine,
  smooth_freq: Lowpass,
}

impl Repeat {
  pub fn new(sample_rate: f64) -> Self {
    Self {
      delay: DelayLine::new(sample_rate as usize * 10, sample_rate),
      smooth_freq: Lowpass::new(sample_rate),
    }
  }

  fn simulate_feedback(&self, index: f32, feedback: f32) -> f32 {
    if feedback == 1. {
      1.
    } else {
      f32::powf(feedback, index)
    }
  }

  fn get_delay_time(&self, index: f32, time_in_ms: f32, skew: f32) -> f32 {
    if skew == 1. {
      time_in_ms * index
    } else {
      f32::powf(skew, index) * time_in_ms
    }
  }

  fn repeat(&mut self, freq: f32, repeats: u32, feedback: f32, skew: f32) -> f32 {
    let time_in_ms = 1000. / freq;
    let mut out = 0f32;
    for i in 1..(repeats - 1) {
      let index = i as f32;
      let multiplication = self.simulate_feedback(index, feedback);
      let time = self.get_delay_time(index, time_in_ms, skew);
      out += self.delay.read(time, "step") * multiplication
    }
    out
  }

  pub fn run(&mut self, input: f32, freq: f32, repeats: f32, feedback: f32, skew: f32) -> f32 {
    self.delay.write(input);
    let frequency = self.smooth_freq.run(freq, 3.);
    let repeated = self.repeat(frequency, repeats as u32, feedback, skew);
    input + repeated
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn feedback() {
    let repeater = Repeat::new(44100.);
    assert_eq!(repeater.simulate_feedback(1.0, 1.0), 1.0);
    assert_eq!(repeater.simulate_feedback(2.0, 1.0), 1.0);
    assert_eq!(repeater.simulate_feedback(3.0, 1.0), 1.0);

    assert_eq!(repeater.simulate_feedback(1.0, 2.0), 2.0);
    assert_eq!(repeater.simulate_feedback(2.0, 2.0), 4.0);
    assert_eq!(repeater.simulate_feedback(3.0, 2.0), 8.0);

    assert_eq!(repeater.simulate_feedback(1.0, 0.5), 0.5);
    assert_eq!(repeater.simulate_feedback(2.0, 0.5), 0.25);
    assert_eq!(repeater.simulate_feedback(3.0, 0.5), 0.125);
  }

  #[test]
  fn delay_time() {
    let repeater = Repeat::new(44100.);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, 1.0), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, 1.0), 200.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, 1.0), 300.0);

    assert_eq!(repeater.get_delay_time(1.0, 100.0, 2.0), 200.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, 2.0), 400.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, 2.0), 800.0);

    assert_eq!(repeater.get_delay_time(1.0, 100.0, 0.5), 50.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, 0.5), 25.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, 0.5), 12.5);
  }
}
