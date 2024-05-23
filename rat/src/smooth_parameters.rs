mod ramp_smooth;
use ramp_smooth::RampSmooth;

pub struct SmoothParameters {
  smooth_distortion: RampSmooth,
  smooth_filter: RampSmooth,
  smooth_volume: RampSmooth,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_distortion: RampSmooth::new(sample_rate, 20.),
      smooth_filter: RampSmooth::new(sample_rate, 20.),
      smooth_volume: RampSmooth::new(sample_rate, 20.),
    }
  }

  pub fn initialize(&mut self, distortion: f32, filter: f32, volume: f32) {
    self.smooth_distortion.initialize(distortion);
    self.smooth_filter.initialize(filter);
    self.smooth_volume.initialize(volume);
  }

  pub fn process(&mut self, distortion: f32, filter: f32, volume: f32) -> (f32, f32, f32) {
    (
      self.smooth_distortion.process(distortion),
      self.smooth_filter.process(filter),
      self.smooth_volume.process(volume),
    )
  }
}
