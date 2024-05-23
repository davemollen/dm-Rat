use crate::shared::one_pole_filter::OnePoleFilter;

const MAX_DB_AT_ONE_HZ: f32 = 120.9794;
const DECREASE_PER_DECADE: f32 = 20.;

pub struct OpAmpCorrection {
  filter: OnePoleFilter,
}

impl OpAmpCorrection {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      filter: OnePoleFilter::new(sample_rate),
    }
  }

  fn atodb(&self, amplitude: f32) -> f32 {
    20. * amplitude.log10()
  }

  fn get_db_gain(&self, distortion: f32) -> f32 {
    let r1 = distortion * 100000.;
    let r2 = 560.;
    let r3 = 47.;
    let gain = 1. + r1 / (r2 * r3 / (r2 + r3));
    self.atodb(gain)
  }

  fn get_cutoff_frequency(&self, distortion: f32) -> f32 {
    let db_gain = self.get_db_gain(distortion);
    let decade_fraction = (MAX_DB_AT_ONE_HZ - db_gain) / DECREASE_PER_DECADE;
    10_f32.powf(decade_fraction)
  }

  pub fn process(&mut self, input: f32, distortion: f32) -> f32 {
    let cutoff_frequency = self.get_cutoff_frequency(distortion);
    self.filter.process(input, cutoff_frequency)
  }
}
