use super::{simd_type::SimdType, Coefficients};

pub trait SimdFir {
  type Float;

  fn new(length: usize) -> Self;
  fn write(&mut self, input: Self::Float);
  fn convolve(&self) -> Self::Float;
  fn process(&mut self, input: Self::Float) -> Self::Float;
}

pub struct FirFilter<T> {
  buffer: Vec<T>,
  coefficients: Vec<T>,
  index: usize,
  mask: usize,
}

impl<T: SimdType> SimdFir for FirFilter<T>
where
  Vec<T>: Coefficients,
{
  type Float = T;

  fn new(length: usize) -> Self {
    debug_assert!(length.is_power_of_two());

    Self {
      buffer: vec![T::splat(0.); length],
      coefficients: Coefficients::new(),
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
