mod smooth;
use smooth::LinearSmooth;
pub use smooth::Smoother;

pub struct Params {
  pub distortion: LinearSmooth,
  pub filter: LinearSmooth,
  pub volume: LinearSmooth,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      distortion: LinearSmooth::new(sample_rate, 20.),
      filter: LinearSmooth::new(sample_rate, 20.),
      volume: LinearSmooth::new(sample_rate, 20.),
      is_initialized: false,
    }
  }

  pub fn set(&mut self, distortion: f32, filter: f32, volume: f32) {
    let distortion = distortion * distortion * distortion;
    let filter = filter * filter * filter;
    let volume = volume * volume * volume;

    if self.is_initialized {
      self.distortion.set_target(distortion);
      self.filter.set_target(filter);
      self.volume.set_target(volume);
    } else {
      self.distortion.reset(distortion);
      self.filter.reset(filter);
      self.volume.reset(volume);
      self.is_initialized = true;
    }
  }
}
