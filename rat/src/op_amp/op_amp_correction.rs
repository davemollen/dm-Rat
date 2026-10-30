use crate::shared::one_pole_filter::OnePoleFilter;

const MAX_GAIN_AT_ONE_HZ: f32 = 1119360.558108; // max gain at 1Hz = 120.9794dB
const MIN_DISTORTION_GAIN: f32 = 1.; // min distortion gain = 0dB
const MAX_DISTORTION_GAIN: f32 = 2307.231003; // max distortion gain = 67.261822dB

pub struct OpAmpCorrection {
  filter: OnePoleFilter,
  freq: f32,
}

impl OpAmpCorrection {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      filter: OnePoleFilter::new(sample_rate),
      freq: 0.,
    }
  }

  pub fn reset(&mut self) {
    self.filter.reset();
  }

  pub fn process(&mut self, input: f32) -> f32 {
    self.filter.process(input, self.freq)
  }

  pub fn set_cutoff_frequency(&mut self, distortion: f32) {
    let gain = distortion * (MAX_DISTORTION_GAIN - MIN_DISTORTION_GAIN) + MIN_DISTORTION_GAIN;
    self.freq = MAX_GAIN_AT_ONE_HZ / gain;
  }
}
