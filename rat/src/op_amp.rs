mod bilinear_transform;
mod op_amp_correction;
mod third_order_iir_filter;
use bilinear_transform::BilinearTransform;
use op_amp_correction::OpAmpCorrection;
use third_order_iir_filter::ThirdOrderIIRFilter;

const R1: f32 = 100000.;
const C1: f32 = 1e-10;

pub struct OpAmp {
  op_amp: ThirdOrderIIRFilter,
  bilinear_transform: BilinearTransform,
  op_amp_correction: OpAmpCorrection,
}

impl OpAmp {
  const Z1_B0: f32 = 2.72149e-7;
  const Z1_B1: f32 = 0.0027354;
  const Z1_A0: f32 = 6.27638e-9;
  const Z1_A1: f32 = 0.0000069;

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
    let z2_b0 = (distortion * R1).max(1.);
    let z2_a0 = z2_b0 * C1;

    let a0 = Self::Z1_B0 * z2_a0;
    let a1 = Self::Z1_B0 + Self::Z1_B1 * z2_a0;
    let a2 = Self::Z1_B1 + z2_a0;
    let b0 = a0;
    let b1 = a1 + Self::Z1_A0 * z2_b0;
    let b2 = Self::Z1_A1 * z2_b0 + Self::Z1_B1 + z2_a0;

    ([b0, b1, b2, 1.], [a0, a1, a2, 1.])
  }
}

#[cfg(test)]
mod tests {
  use super::OpAmp;

  #[test]
  fn s_domain_coefficients_should_be_correct_for_distortion_at_one() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [2.72149e-12, 0.000627937503, 0.6927454, 1.0],
      [2.72149e-12, 2.99503e-07, 0.0027454, 1.0],
    );
    assert_eq!(op_amp.get_s_domain_coefficients(1.), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_distortion_at_one_tenth() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [2.72149e-13, 6.30386844e-05, 0.0717364, 1.0],
      [2.72149e-13, 2.748844e-07, 0.0027364, 1.0],
    );
    assert_eq!(op_amp.get_s_domain_coefficients(0.1), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_distortion_at_one_hundredth() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [2.72149e-14, 6.54880254e-06, 0.009635499999999998, 1.0],
      [2.72149e-14, 2.7242254e-07, 0.0027354999999999997, 1.0],
    );
    assert_eq!(op_amp.get_s_domain_coefficients(0.01), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_distortion_at_one_thousandth() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [2.72149e-15, 8.99814354e-07, 0.00342541, 1.0],
      [2.72149e-15, 2.72176354e-07, 0.0027354099999999998, 1.0],
    );
    assert_eq!(op_amp.get_s_domain_coefficients(0.001), coeffs)
  }

  #[test]
  fn s_domain_coefficients_should_be_correct_for_distortion_at_one_hundred_thousandth() {
    let op_amp = OpAmp::new(44100.);

    let coeffs: ([f32; 4], [f32; 4]) = (
      [
        2.7214899999999998e-17,
        2.7842565354e-07,
        0.0027423000999999996,
        1.0,
      ],
      [2.7214899999999998e-17, 2.7214927354e-07, 0.0027354001, 1.0],
    );
    assert_eq!(op_amp.get_s_domain_coefficients(0.00001), coeffs)
  }
}
