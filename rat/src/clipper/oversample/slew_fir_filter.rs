use super::{fir_filter::SimdFir, simd_type::SimdType, SlewCoefficients};

pub struct SlewFirFilter<T> {
  buffer: Vec<T>,
  coefficients: Vec<T>,
  index: usize,
  mask: usize,
}

impl<T: SimdType> SimdFir for SlewFirFilter<T>
where
  Vec<T>: SlewCoefficients,
{
  type Float = T;

  fn new(length: usize) -> Self {
    debug_assert!(length.is_power_of_two());

    Self {
      buffer: vec![T::splat(0.); length],
      coefficients: SlewCoefficients::new(),
      index: 0,
      mask: length - 1,
    }
  }

  fn process(&mut self, input: Self::Float) -> T {
    self.write(input);
    self.convolve()
  }

  fn write(&mut self, input: Self::Float) {
    self.buffer[self.index] = input;
    self.index = self.index + 1 & self.mask;
  }

  fn convolve(&self) -> T {
    let coefficients = &self.coefficients;

    let (front, back) = self.buffer.split_at(self.index);
    back
      .iter()
      .chain(front)
      .zip(coefficients)
      .map(|(input, coeff)| *input * *coeff)
      .sum::<T>()
  }
}
