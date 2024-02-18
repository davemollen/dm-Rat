mod ramp_smooth;
use ramp_smooth::RampSmooth;

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
    let ramp_time = 50.;
    (
      self.filters[0].process(distortion, ramp_time),
      self.filters[1].process(filter, ramp_time),
      self.filters[2].process(volume, ramp_time),
    )
  }
}
