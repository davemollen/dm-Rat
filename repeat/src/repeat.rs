use super::delay_line::DelayLine;
use super::lowpass::Lowpass;
use std::f32;

pub struct Repeat {
  delay: DelayLine,
  smooth_freq: Lowpass,
  smooth_skew: Lowpass,
  previous_time: f32,
}

impl Repeat {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay: DelayLine::new(sample_rate as usize * 10, sample_rate),
      smooth_freq: Lowpass::new(sample_rate),
      smooth_skew: Lowpass::new(sample_rate),
      previous_time: 0.0,
    }
  }

  fn reverse_indices(&self, index: f32, input: f32, repeats: u32) -> f32 {
    if input.signum() == 1. {
      index
    } else {
      repeats as f32 - index - 1.0
    }
  }

  fn simulate_feedback(&self, index: f32, feedback: f32, repeats: u32) -> f32 {
    let feedback_index = self.reverse_indices(index, feedback, repeats);
    let absolute_feedback = feedback.abs();
    if absolute_feedback == 1. {
      1.
    } else {
      f32::powf(absolute_feedback, feedback_index)
    }
  }

  fn get_delay_time(&mut self, index: f32, time: f32, skew: f32) -> f32 {
    if index == 0. {
      self.previous_time = 0.0;
      0.0
    } else if skew == 0. {
      time * index
    } else {
      let exponential_skew = f32::powf(2.0, skew);
      let delay_time = if index == 1. {
        f32::powf(exponential_skew, index - 1.) * time
      } else {
        f32::powf(exponential_skew, index - 1.) * time + self.previous_time
      };
      self.previous_time = delay_time;
      delay_time
    }
  }

  fn repeat(&mut self, input: f32, freq: f32, repeats: u32, feedback: f32, skew: f32) -> f32 {
    let time_in_ms = 1000. / freq;
    let mut out = 0f32;
    for i in 0..repeats {
      let index = i as f32;
      let multiplication = self.simulate_feedback(index, feedback, repeats);
      if i == 0 {
        out += input * multiplication;
      } else {
        let time = self.get_delay_time(index, time_in_ms, skew);
        out += self.delay.read(time, "step") * multiplication;
      }
    }
    out
  }

  pub fn run(&mut self, input: f32, freq: f32, repeats: f32, feedback: f32, skew: f32) -> f32 {
    self.delay.write(input);
    let smoothed_freq = self.smooth_freq.run(freq, 3.);
    let smoothed_skew = self.smooth_skew.run(skew, 3.);
    let repeated = self.repeat(
      input,
      smoothed_freq,
      repeats as u32,
      feedback,
      smoothed_skew,
    );
    repeated
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn feedback() {
    let repeater = Repeat::new(44100.);
    assert_eq!(repeater.simulate_feedback(0.0, 1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(1.0, 1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(2.0, 1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(3.0, 1.0, 4), 1.0);

    assert_eq!(repeater.simulate_feedback(0.0, 2.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(1.0, 2.0, 4), 2.0);
    assert_eq!(repeater.simulate_feedback(2.0, 2.0, 4), 4.0);
    assert_eq!(repeater.simulate_feedback(3.0, 2.0, 4), 8.0);

    assert_eq!(repeater.simulate_feedback(0.0, 0.5, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(1.0, 0.5, 4), 0.5);
    assert_eq!(repeater.simulate_feedback(2.0, 0.5, 4), 0.25);
    assert_eq!(repeater.simulate_feedback(3.0, 0.5, 4), 0.125);

    assert_eq!(repeater.simulate_feedback(0.0, -1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(1.0, -1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(2.0, -1.0, 4), 1.0);
    assert_eq!(repeater.simulate_feedback(3.0, -1.0, 4), 1.0);

    assert_eq!(repeater.simulate_feedback(0.0, -2.0, 4), 8.0);
    assert_eq!(repeater.simulate_feedback(1.0, -2.0, 4), 4.0);
    assert_eq!(repeater.simulate_feedback(2.0, -2.0, 4), 2.0);
    assert_eq!(repeater.simulate_feedback(3.0, -2.0, 4), 1.0);

    assert_eq!(repeater.simulate_feedback(0.0, -0.5, 4), 0.125);
    assert_eq!(repeater.simulate_feedback(1.0, -0.5, 4), 0.25);
    assert_eq!(repeater.simulate_feedback(2.0, -0.5, 4), 0.5);
    assert_eq!(repeater.simulate_feedback(3.0, -0.5, 4), 1.0);
  }

  #[test]
  fn delay_time() {
    let mut repeater = Repeat::new(44100.);
    assert_eq!(repeater.get_delay_time(0.0, 100.0, 0.0), 0.0);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, 0.0), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, 0.0), 200.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, 0.0), 300.0);

    assert_eq!(repeater.get_delay_time(0.0, 100.0, 1.0), 0.0);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, 1.0), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, 1.0), 300.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, 1.0), 700.0);

    assert_eq!(repeater.get_delay_time(0.0, 100.0, 0.5), 0.0);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, 0.5), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, 0.5), 241.42136);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, 0.5), 441.42133);

    assert_eq!(repeater.get_delay_time(0.0, 100.0, -0.5), 0.0);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, -0.5), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, -0.5), 170.71068);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, -0.5), 220.71068);

    assert_eq!(repeater.get_delay_time(0.0, 100.0, -1.0), 0.0);
    assert_eq!(repeater.get_delay_time(1.0, 100.0, -1.0), 100.0);
    assert_eq!(repeater.get_delay_time(2.0, 100.0, -1.0), 150.0);
    assert_eq!(repeater.get_delay_time(3.0, 100.0, -1.0), 175.0);
  }
}
