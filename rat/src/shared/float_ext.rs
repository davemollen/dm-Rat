pub trait FloatExt {
  fn mix(self, right: Self, factor: Self) -> Self;
  fn cubic_spline_curve(self) -> Self;
}

impl FloatExt for f32 {
  #[inline(always)]
  fn mix(self, right: Self, factor: Self) -> Self {
    self + (right - self) * factor
  }

  #[inline(always)]
  fn cubic_spline_curve(self) -> Self {
    self * self * (3.0 - 2.0 * self)
  }
}

#[cfg(test)]
mod tests {
  use super::FloatExt;

  #[test]
  fn mix() {
    assert_eq!((1f32).mix(0., 0.), 1.);
    assert_eq!((1f32).mix(0., 0.5), 0.5);
    assert_eq!((1f32).mix(0., 1.), 0.);
  }
}
