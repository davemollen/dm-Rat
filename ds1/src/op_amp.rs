pub struct OpAmp {
  z1: f32,
  z2: f32,
  sample_rate: f32,
}

impl OpAmp {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      z1: 0.,
      z2: 0.,
      sample_rate,
    }
  }

  pub fn process(&mut self, input: f32, dist: f32) -> f32 {
    let dist = dist.min(0.00001);
    let double_sample_rate = self.sample_rate * 2.;
    let squared_double_sample_rate = double_sample_rate * double_sample_rate;

    let rt = dist * 100000.;
    let rb = ((1. - dist) * 100000.) + 4700.;
    let cz = 0.000001;
    let cc = 0.00000000025;

    let a = (rt * rb * cz * cc).recip();
    let c = (rt * cc).recip() + (rb * cz).recip();
    let b = c + (rb * cc).recip();

    let b0 = a + c * double_sample_rate + squared_double_sample_rate;
    let a0 = (a + b * double_sample_rate + squared_double_sample_rate) / b0;
    let a1 = (2. * a - 2. * squared_double_sample_rate) / b0;
    let a2 = (a - b * double_sample_rate + squared_double_sample_rate) / b0;
    let b1 = a1;
    let b2 = (a - c * double_sample_rate + squared_double_sample_rate) / b0;

    let y = input * a0 + self.z1;
    self.z1 = input * a1 + self.z2 - b1 * y;
    self.z2 = input * a2 - b2 * y;

    y
  }
}
