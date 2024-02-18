use crate::shared::lowpass_filter::LowpassFilter;

pub struct OpAmpCorrection {
  filter: LowpassFilter,
}

impl OpAmpCorrection {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      filter: LowpassFilter::new(sample_rate),
    }
  }

  fn atodb(&self, amplitude: f32) -> f32 {
    20. * amplitude.log10()
  }

  fn get_db_gain(&self, distortion: f32) -> f32 {
    let r1 = distortion * 100000.;
    let r2 = 560.;
    let r3 = 47.;
    let gain = 1. + r1 / (r2 * r2 / (r2 + r3));
    self.atodb(gain)
  }

  fn get_cutoff_frequency(&self, distortion: f32) -> f32 {
    let db_gain = self.get_db_gain(distortion);
    let max_db_at_one_hz = 120.9794;
    let decrease_per_decade = 20.;
    let decade_fraction = (max_db_at_one_hz - db_gain) / decrease_per_decade;
    10_f32.powf(decade_fraction)
  }

  pub fn process(&mut self, input: f32, distortion: f32) -> f32 {
    let cutoff_frequency = self.get_cutoff_frequency(distortion);
    self.filter.process(input, cutoff_frequency)
  }
}
