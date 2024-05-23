use crate::shared::one_pole_filter::OnePoleFilter;

const MAX_GAIN_AT_ONE_HZ: f32 = 1119360.558108; // max gain at 1Hz = 120.9794dB
const MIN_DISTORTION_GAIN: f32 = 1.; // min distortion gain = 0dB
const MAX_DISTORTION_GAIN: f32 = 2307.231003; // max distortion gain = 67.261822dB

pub struct OpAmpCorrection {
  filter: OnePoleFilter,
}

impl OpAmpCorrection {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      filter: OnePoleFilter::new(sample_rate),
    }
  }

  fn get_cutoff_frequency(&self, distortion: f32) -> f32 {
    let gain = distortion * (MAX_DISTORTION_GAIN - MIN_DISTORTION_GAIN) + MIN_DISTORTION_GAIN;
    MAX_GAIN_AT_ONE_HZ / gain
  }

  pub fn process(&mut self, input: f32, distortion: f32) -> f32 {
    let cutoff_frequency = self.get_cutoff_frequency(distortion);
    self.filter.process(input, cutoff_frequency)
  }
}
