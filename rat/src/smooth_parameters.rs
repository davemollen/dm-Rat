mod ramp_smooth;
use ramp_smooth::RampSmooth;

const RAMP_TIME: f32 = 50.;

pub struct SmoothParameters {
  filters: [RampSmooth; 3],
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      filters: [RampSmooth::new(sample_rate); 3],
    }
  }

  pub fn process(&mut self, distortion: f32, filter: f32, volume: f32) -> (f32, f32, f32) {
    (
      self.filters[0].process(distortion, RAMP_TIME),
      self.filters[1].process(filter, RAMP_TIME),
      self.filters[2].process(volume, RAMP_TIME),
    )
  }
}
