pub struct FirFilter {
  buffer: Vec<[f32; 8]>,
  coefficients: Vec<[f32; 8]>,
  index: usize,
  mask: usize,
}

impl FirFilter {
  pub fn new(coefficients: Vec<[f32; 8]>) -> Self {
    let length = coefficients.len();
    debug_assert!(length.is_power_of_two());

    Self {
      buffer: vec![[0.; 8]; length],
      coefficients,
      index: 0,
      mask: length - 1,
    }
  }

  pub fn process(&mut self, input: [f32; 8]) -> [f32; 8] {
    self.write(input);
    self.convolve()
  }

  fn write(&mut self, input: [f32; 8]) {
    self.buffer[self.index] = input;
    self.index = self.index + 1 & self.mask;
  }

  fn convolve(&self) -> [f32; 8] {
    let coefficients = &self.coefficients;

    let (front, back) = self.buffer.split_at(self.index);
    back
      .iter()
      .chain(front)
      .zip(coefficients)
      .fold([0.; 8], |mut result, (input, coeff)| {
        for i in 0..8 {
          result[i] += input[i] * coeff[i];
        }
        result
      })
  }
}