use std::simd::f32x8;

pub struct FirFilter {
  buffer: Vec<f32x8>,
  coefficients: Vec<f32x8>,
  index: usize,
  mask: usize,
}

impl FirFilter {
  pub fn new(coefficients: Vec<f32x8>) -> Self {
    let length = coefficients.len();
    debug_assert!(length.is_power_of_two());

    Self {
      buffer: vec![f32x8::splat(0.); length],
      coefficients,
      index: 0,
      mask: length - 1,
    }
  }

  pub fn process(&mut self, input: f32x8) -> f32x8 {
    self.write(input);
    self.convolve()
  }

  fn write(&mut self, input: f32x8) {
    self.buffer[self.index] = input;
    self.index = self.index + 1 & self.mask;
  }

  fn convolve(&self) -> f32x8 {
    let coefficients = &self.coefficients;

    let (front, back) = self.buffer.split_at(self.index);
    back
      .iter()
      .chain(front)
      .zip(coefficients)
      .map(|(input, coeff)| *input * *coeff)
      .sum()
  }
}
