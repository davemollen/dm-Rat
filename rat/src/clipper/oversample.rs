mod coefficients;
mod fir_filter;
mod slew_coefficients;
use {
  coefficients::Coefficients,
  fir_filter::FirFilter,
  slew_coefficients::SlewCoefficients,
  std::simd::{f32x8, num::SimdFloat},
};

const OVERSAMPLE_FACTOR: f32 = 8.;

pub struct Oversample {
  upsample_fir: FirFilter,
  downsample_fir: FirFilter,
}

impl Oversample {
  pub fn new() -> Self {
    Self {
      upsample_fir: FirFilter::new(SlewCoefficients::new()),
      downsample_fir: FirFilter::new(Coefficients::new()),
    }
  }

  pub fn process<F>(&mut self, input: f32, callback: F) -> f32
  where
    F: Fn(f32x8) -> f32x8,
  {
    let upsampled = self.upsample(input);
    let processed = self.run_upsampled_process(upsampled, callback);
    self.downsample(processed)
  }

  fn upsample(&mut self, input: f32) -> f32x8 {
    self
      .upsample_fir
      .process(f32x8::splat(input * OVERSAMPLE_FACTOR))
  }

  fn run_upsampled_process<F>(&mut self, input: f32x8, callback: F) -> f32x8
  where
    F: Fn(f32x8) -> f32x8,
  {
    callback(input)
  }

  fn downsample(&mut self, input: f32x8) -> f32 {
    self.downsample_fir.process(input).reduce_sum()
  }
}
