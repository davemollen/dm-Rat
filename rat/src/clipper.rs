mod coefficients;
mod fir_filter;
mod lookup_table;
mod slew_coefficients;
use {
  coefficients::Coefficients, fir_filter::FirFilter, lookup_table::DIODE_TABLE,
  slew_coefficients::SlewCoefficients,
};

const OVERSAMPLE_FACTOR: f32 = 8.;

pub struct Clipper {
  upsample_fir: FirFilter,
  downsample_fir: FirFilter,
}

impl Clipper {
  const SIZE: usize = DIODE_TABLE.len() - 1;
  const HALF_SIZE: f32 = DIODE_TABLE.len() as f32 * 0.5;

  pub fn new() -> Self {
    Self {
      upsample_fir: FirFilter::new(Coefficients::new()),
      downsample_fir: FirFilter::new(SlewCoefficients::new()),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let upsampled = self.upsample_fir.process([input * OVERSAMPLE_FACTOR; 8]);
    let clipped = upsampled.map(|x| Self::simulate_diode_clipping(x));
    self.downsample_fir.process(clipped).into_iter().sum()
  }

  fn simulate_diode_clipping(x: f32) -> f32 {
    let x = (x * 0.25) * Self::HALF_SIZE + Self::HALF_SIZE;
    let index = x.trunc();
    let frac = x - index;
    let i = index as usize;

    (DIODE_TABLE[i.clamp(0, Self::SIZE)] * (1. - frac)
      + DIODE_TABLE[(i + 1).clamp(0, Self::SIZE)] * frac)
      * 0.5
  }
}
