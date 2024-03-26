mod oversample;
use oversample::Oversample;
use simba::simd::{f32x8, SimdComplexField};

pub struct Clipper {
  oversample: Oversample<f32x8>,
}

impl Clipper {
  pub fn new() -> Self {
    Self {
      oversample: Oversample::new(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    self.oversample.process(input, |x| {
      // x.simd_tanh()

      let squared_self = x * x;
      let z = f32x8::splat(135135.);

      let a = x
        * (z
          + squared_self
            * (f32x8::splat(17325.) + squared_self * (f32x8::splat(378.) + squared_self)));
      let b = z
        + squared_self
          * (f32x8::splat(62370.) + squared_self * (f32x8::splat(3150.) + squared_self * 28.));
      a / b
    }) * 0.3728
  }
}
