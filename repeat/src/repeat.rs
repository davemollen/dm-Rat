use super::delay_line::DelayLine;
use super::lowpass::Lowpass;
use std::f32;

pub struct Repeat {
  delay: DelayLine,
  smooth_freq: Lowpass,
  smooth_skew: Lowpass,
}

impl Repeat {
  pub fn new(sample_rate: f64) -> Self {
    Self {
      delay: DelayLine::new(sample_rate as usize * 10, sample_rate),
      smooth_freq: Lowpass::new(sample_rate),
      smooth_skew: Lowpass::new(sample_rate),
    }
  }

  fn simulate_feedback(&self, index: f32, feedback: f32) -> f32 {
    if feedback == 1. {
      1.
    } else {
      f32::powf(feedback, index)
    }
  }

  fn get_delay_time(&self, index: f32, time: f32, skew: f32) -> f32 {
    if skew == 0. {
      time * index
    } else {
      let exponential_skew = f32::powf(2.0, skew);
      if exponential_skew > 1.0 {
        f32::powf(exponential_skew, index) * time - time
      } else {
        (1. - f32::powf(exponential_skew, index - 1.)) * time + time
      }
    }
  }

  fn reverse_indices(&self, index: f32, input: f32, repeats: u32) -> f32 {
    if input.signum() == 1. {
      index
    } else {
      repeats as f32 - index - 1.0
    }
  }

  fn repeat(&mut self, freq: f32, repeats: u32, feedback: f32, skew: f32) -> f32 {
    let time_in_ms = 1000. / freq;
    let mut out = 0f32;
    for i in 0..repeats {
      let index = i as f32;
      let feedback_index = self.reverse_indices(index, feedback, repeats);
      let multiplication = self.simulate_feedback(feedback_index, feedback.abs());
      let time = self.get_delay_time(index, time_in_ms, skew);
      out += self.delay.read(time, "step") * multiplication
    }
    out
  }

  pub fn run(&mut self, input: f32, freq: f32, repeats: f32, feedback: f32, skew: f32) -> f32 {
    self.delay.write(input);
    let smoothed_freq = self.smooth_freq.run(freq, 3.);
    let smoothed_skew = self.smooth_skew.run(skew, 3.);
    let repeated = self.repeat(smoothed_freq, repeats as u32, feedback, smoothed_skew);
    repeated
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn feedback() {
    let repeater = Repeat::new(44100.);
    assert_eq!(repeater.simulate_feedback(0.0, 1.0), 1.0);
    assert_eq!(repeater.simulate_feedback(1.0, 1.0), 1.0);
    assert_eq!(repeater.simulate_feedback(2.0, 1.0), 1.0);
    assert_eq!(repeater.simulate_feedback(3.0, 1.0), 1.0);

    assert_eq!(repeater.simulate_feedback(0.0, 2.0), 1.0);
    assert_eq!(repeater.simulate_feedback(1.0, 2.0), 2.0);
    assert_eq!(repeater.simulate_feedback(2.0, 2.0), 4.0);
    assert_eq!(repeater.simulate_feedback(3.0, 2.0), 8.0);

    assert_eq!(repeater.simulate_feedback(0.0, 0.5), 1.0);
    assert_eq!(repeater.simulate_feedback(1.0, 0.5), 0.5);
    assert_eq!(repeater.simulate_feedback(2.0, 0.5), 0.25);
    assert_eq!(repeater.simulate_feedback(3.0, 0.5), 0.125);
  }

  #[test]
  fn delay_time() {
    let repeater = Repeat::new(44100.);
    assert_eq!(repeater.get_delay_time(0.0, 100.0, 0.0), 0.0);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, 0.0), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, 0.0), 200.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, 0.0), 300.0);

    assert_eq!(repeater.get_delay_time(0.0, 100.0, 1.0), 0.0);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, 1.0), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, 1.0), 300.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, 1.0), 700.0);

    assert_eq!(repeater.get_delay_time(0.0, 100.0, 0.0), 0.0);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, -1.0), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, -1.0), 150.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, -1.0), 175.0);
  }

  #[test]
  fn reversed_feedback() {
    let repeater = Repeat::new(44100.);
    let repeats = 4;
    let expected_results = vec![0.125, 0.25, 0.5, 1.0];

    for i in 0..repeats {
      let feedback_index = repeater.reverse_indices(i as f32, -1.0, repeats);
      let expected_result = expected_results[i as usize];
      assert_eq!(
        repeater.simulate_feedback(feedback_index, 0.5),
        expected_result
      );
    }
  }
}
