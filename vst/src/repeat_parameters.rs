use vst::{plugin::PluginParameters, util::AtomicFloat};

pub struct RepeatParameters {
  pub freq: AtomicFloat,
  pub repeats: AtomicFloat,
  pub feedback: AtomicFloat,
  pub skew: AtomicFloat,
}

impl Default for RepeatParameters {
  fn default() -> Self {
    Self {
      freq: AtomicFloat::new(4.0),
      repeats: AtomicFloat::new(4.0),
      feedback: AtomicFloat::new(0.9),
      skew: AtomicFloat::new(0.5),
    }
  }
}

impl PluginParameters for RepeatParameters {
  fn get_parameter(&self, index: i32) -> f32 {
    match index {
      0 => ((self.freq.get() - 0.1) / 49.9).powf(0.333333),
      1 => (self.repeats.get() - 1.0) / 31.0,
      2 => self.feedback.get(),
      3 => self.skew.get(),
      _ => 0.0,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    match index {
      0 => format!("{:.2} hz", self.freq.get()),
      1 => format!("{}", self.repeats.get()),
      2 => format!("{:.2}%", self.feedback.get() * 250.0 - 125.0),
      3 => format!("{:.2}%", self.skew.get() * 200.0 - 100.0),
      _ => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => "Frequency",
      1 => "Repeats",
      2 => "Feedback",
      3 => "Skew",
      _ => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match index {
      0 => self.freq.set(val.powf(3.) * 49.9 + 0.1),
      1 => self.repeats.set((val * 31.0 + 1.0).floor()),
      2 => self.feedback.set(val),
      3 => self.skew.set(val),
      _ => (),
    }
  }
}
