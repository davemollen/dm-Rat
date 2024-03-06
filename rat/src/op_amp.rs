mod bilinear_transform;
mod op_amp_correction;
mod third_order_iir_filter;
use bilinear_transform::BilinearTransform;
use op_amp_correction::OpAmpCorrection;
use third_order_iir_filter::ThirdOrderIIRFilter;

pub struct OpAmp {
  op_amp: ThirdOrderIIRFilter,
  bilinear_transform: BilinearTransform,
  op_amp_correction: OpAmpCorrection,
}

impl OpAmp {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: ThirdOrderIIRFilter::new(),
      bilinear_transform: BilinearTransform::new(sample_rate),
      op_amp_correction: OpAmpCorrection::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, distortion: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients(distortion);
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
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
}
