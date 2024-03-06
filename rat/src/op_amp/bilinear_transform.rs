pub struct BilinearTransform {
  s: [f32; 3],
}

impl BilinearTransform {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: [t / 2., t * t / 4., t * t * t / 8.],
    }
  }

  fn bilinear_transform(&self, mut x: [f32; 4]) -> [f32; 4] {
    x[1] *= self.s[0];
    x[2] *= self.s[1];
    x[3] *= self.s[2];

    [
      x[0] + x[1] + x[2] + x[3],
      -3. * x[0] - x[1] + x[2] + 3. * x[3],
      3. * x[0] - x[1] - x[2] + 3. * x[3],
      -x[0] + x[1] - x[2] + x[3],
    ]
  }

  pub fn process(&self, (b, a): ([f32; 4], [f32; 4])) -> ([f32; 4], [f32; 4]) {
    let b = self.bilinear_transform(b);
    let a = self.bilinear_transform(a);
    (b.map(|x| x / a[0]), a.map(|x| x / a[0]))
  }
}

#[cfg(test)]
mod tests {
  use super::BilinearTransform;

  #[test]
  fn bilinear_transform_should_be_correct() {
    let bilinear_transform = BilinearTransform::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [1., 2.27816380e+08, 2.54546370e+11, 3.67445774e+11],
      [1., 1.10051112e+05, 1.00878563e+09, 3.67445774e+11],
    );
    assert_eq!(
      bilinear_transform.process(coeffs),
      (
        [1100.38731292, -1073.70700939, -1098.70429603, 1072.02579416],
        [1., -1.73109163, 0.6830169, 0.0498764],
      )
    );
  }
}
