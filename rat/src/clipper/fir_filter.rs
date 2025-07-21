use std::array;

const OVERSAMPLE_FACTOR: f32 = 8.;

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

  pub fn upsample(&mut self, input: f32) -> [f32; 8] {
    self.write([input, 0., 0., 0., 0., 0., 0., 0.]);

    array::from_fn(|j| {
      (0..self.buffer.len())
        .map(|i| {
          let buffer_index = (self.index + self.buffer.len() - i) & self.mask;
          self.buffer[j][buffer_index] * self.coefficients[j][i]
        })
        .sum()
    })
  }

  fn write(&mut self, input: [f32; 8]) {
    self.index = self.index + 1 & self.mask;
    self.buffer[self.index] = input;
  }
}
