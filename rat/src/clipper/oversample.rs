mod coefficients;
pub use coefficients::Coefficients;
mod slew_coefficients;
pub use slew_coefficients::SlewCoefficients;
mod fir_filter;
use fir_filter::{FirFilter, SimdFir};
mod slew_fir_filter;
use slew_fir_filter::SlewFirFilter;
mod simd_type;
use simd_type::SimdType;

pub struct Oversample<T> {
  upsample_fir: SlewFirFilter<T>,
  downsample_fir: FirFilter<T>,
  oversample_factor: usize,
}

impl<T: SimdType> Oversample<T>
where
  Vec<T>: SlewCoefficients,
  Vec<T>: Coefficients,
{
  pub fn new() -> Self {
    let oversample_factor = T::oversample_factor();

    Self {
      upsample_fir: SlewFirFilter::new(16),
      downsample_fir: FirFilter::new(16),
      oversample_factor,
    }
  }

  pub fn process<F>(&mut self, input: f32, callback: F) -> f32
  where
    F: Fn(T) -> T,
  {
    let upsampled = self.upsample(input);
    let processed = self.run_upsampled_process(upsampled, callback);
    self.downsample(processed)
  }

  fn upsample(&mut self, input: f32) -> T {
    self
      .upsample_fir
      .process(SimdType::splat(input * self.oversample_factor as f32))
  }

  fn run_upsampled_process<F>(&mut self, input: T, callback: F) -> T
  where
    F: Fn(T) -> T,
  {
    callback(input)
  }

  fn downsample(&mut self, input: T) -> f32 {
    self.downsample_fir.process(input).reduce_sum()
  }
}
