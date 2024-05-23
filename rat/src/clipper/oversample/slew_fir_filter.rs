use super::{fir_filter::SimdFir, simd_type::SimdType, SlewCoefficients};

pub struct SlewFirFilter<T> {
  buffer: Vec<T>,
  coefficients: Vec<T>,
}

impl<T: SimdType> SimdFir for SlewFirFilter<T>
where
  Vec<T>: SlewCoefficients,
{
  type Float = T;

  fn new(length: usize) -> Self {
    Self {
      buffer: vec![T::splat(0.); length],
      coefficients: SlewCoefficients::new(),
    }
  }

  fn process(&mut self, input: Self::Float) -> T {
    self.write(input);
    self.convolve()
  }

  fn write(&mut self, input: Self::Float) {
    self.buffer.insert(0, input);
    self.buffer.pop();
  }

  fn convolve(&self) -> T {
    let coefficients = &self.coefficients;

    self
      .buffer
      .iter()
      .zip(coefficients)
      .map(|(input, coeff)| *input * *coeff)
      .sum::<T>()
  }
}
