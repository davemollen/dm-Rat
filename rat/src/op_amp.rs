mod op_amp_correction;
mod third_order_iir_filter;
use op_amp_correction::OpAmpCorrection;
use third_order_iir_filter::ThirdOrderIIRFilter;

pub struct OpAmp {
  s1: f32,
  s2: f32,
  s3: f32,
  op_amp: ThirdOrderIIRFilter,
  op_amp_correction: OpAmpCorrection,
}

impl OpAmp {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s1: t / 2.,
      s2: t * t / 4.,
      s3: t * t * t / 8.,
      op_amp: ThirdOrderIIRFilter::new(),
      op_amp_correction: OpAmpCorrection::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, distortion: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients(distortion);
    let z_domain_coefficients = self.bilinear_transform(s_domain_coefficients);
    let ideal_op_amp_output = self.op_amp.process(input, z_domain_coefficients);
    self
      .op_amp_correction
      .process(ideal_op_amp_output, distortion)
  }

  fn get_s_domain_coefficients(&self, distortion: f32) -> ([f32; 4], [f32; 4]) {
    let z2_b0 = (distortion * 100000.).max(1.);
    let z2_a0 = z2_b0 * 1e-10;

    let z1_b0 = 2.72149e-7;
    let z1_b1 = 0.0027354;
    let z1_a0 = 6.27638e-9;
    let z1_a1 = 0.0000069;

    let a0 = z1_b0 * z2_a0;
    let a1 = z1_b0 + z1_b1 * z2_a0;
    let a2 = z1_b1 + z2_a0;
    let b0 = a0;
    let b1 = a1 + z1_a0 * z2_b0;
    let b2 = z1_a1 * z2_b0 + z1_b1 + z2_a0;

    (
      [b0 / a0, b1 / a0, b2 / a0, 1. / a0],
      [1., a1 / a0, a2 / a0, 1. / a0],
    )
  }

  fn bilinear_transform(&self, (mut b, mut a): ([f32; 4], [f32; 4])) -> ([f32; 4], [f32; 4]) {
    b[1] *= self.s1;
    b[2] *= self.s2;
    b[3] *= self.s3;

    let b0 = b[0] + b[1] + b[2] + b[3];
    let b1 = -3. * b[0] - b[1] + b[2] + 3. * b[3];
    let b2 = 3. * b[0] - b[1] - b[2] + 3. * b[3];
    let b3 = -b[0] + b[1] - b[2] + b[3];

    a[1] *= self.s1;
    a[2] *= self.s2;
    a[3] *= self.s3;

    let a0 = a[0] + a[1] + a[2] + a[3];
    let a1 = -3. * a[0] - a[1] + a[2] + 3. * a[3];
    let a2 = 3. * a[0] - a[1] - a[2] + 3. * a[3];
    let a3 = -a[0] + a[1] - a[2] + a[3];

    (
      [b0 / a0, b1 / a0, b2 / a0, b3 / a0],
      [a0 / a0, a1 / a0, a2 / a0, a3 / a0],
    )
  }
}

#[cfg(test)]
mod tests {
  use super::OpAmp;

  #[test]
  fn s_domain_coefficients_should_be_correct_for_distortion_at_one() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [1.0, 230732960.0, 254546380000.0, 367445770000.0],
      [1.0, 110051.12, 1008785700.0, 367445770000.0],
    );
    assert_eq!(op_amp.get_s_domain_coefficients(1.), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_distortion_at_one_tenth() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [1., 2.20467465e+08, 2.63592370e+11, 3.67445774e+12],
      [1., 1.01004964e+06, 1.00547862e+10, 3.67445774e+12],
    );
    assert_eq!(op_amp.get_s_domain_coefficients(0.1), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_distortion_at_one_hundredth() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [1., 2.40632889e+08, 3.54052376e+11, 3.67445774e+13],
      [1., 1.00100680e+07, 1.00514792e+11, 3.67445774e+13],
    );
    assert_eq!(op_amp.get_s_domain_coefficients(0.01), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_distortion_at_one_thousandth() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [1.0, 330632960.0, 1258652300000.0, 367445730000000.0],
      [1.0, 100010030.0, 1005114750000.0, 367445730000000.0],
    );
    assert_eq!(op_amp.get_s_domain_coefficients(0.001), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_distortion_at_one_hundred_thousandth() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [1., 1.02306457e+10, 1.00764658e+14, 3.67445774e+16],
      [1., 1.00000000e+10, 1.00511121e+14, 3.67445774e+16],
    );
    assert_eq!(op_amp.get_s_domain_coefficients(0.00001), coeffs)
  }

  #[test]
  fn bilinear_transform_should_be_correct_for_distortion_at_one() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [1., 2.27816380e+08, 2.54546370e+11, 3.67445774e+11],
      [1., 1.10051112e+05, 1.00878563e+09, 3.67445774e+11],
    );
    assert_eq!(
      op_amp.bilinear_transform(coeffs),
      (
        [1100.38731292, -1073.70700939, -1098.70429603, 1072.02579416],
        [1., -1.73109163, 0.6830169, 0.0498764],
      )
    );
  }

  #[test]
  fn bilinear_transform_should_be_correct_for_distortion_at_one_thousandth() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [1., 3.30632852e+08, 1.25865243e+12, 3.67445774e+14],
      [1., 1.00009921e+08, 1.00511485e+12, 3.67445774e+14],
    );
    assert_eq!(
      op_amp.bilinear_transform(coeffs),
      (
        [3.09337424, -2.83738449, -3.08851741, 2.83591541],
        [1., -0.79555309, -0.99514317, 0.79408401],
      )
    );
  }
}
