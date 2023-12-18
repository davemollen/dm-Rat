use vst::plugin::PluginParameters;
mod params;
pub use params::{FloatParam, FloatRange, IntParam, IntRange, Params};
mod formatters;
use formatters::{v2s_f32_digits, v2s_f32_percentage, s2v_f32_percentage};

pub struct RepeatParameters {
  pub freq: FloatParam,
  pub repeats: IntParam,
  pub feedback: FloatParam,
  pub skew: FloatParam,
}

impl Default for RepeatParameters {
  fn default() -> Self {
    Self {
      freq: FloatParam::new(
        "Freq",
        4.,
        0,
        FloatRange::Skewed {
          min: 0.1,
          max: 50.,
          factor: 0.3
        }
      )
      .with_unit(" Hz")
      .with_value_to_string(v2s_f32_digits(2)),

      repeats: IntParam::new(
        "Repeats",
        4,
        1,
        IntRange::Linear {
          min: 1,
          max: 24
        }
      )
      .with_unit(" x"),

      feedback: FloatParam::new(
        "Feedback",
        1.,
        2,
        FloatRange::Linear { 
          min: -1.25, 
          max: 1.25 
        }
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(2))
      .with_string_to_value(s2v_f32_percentage()),

      skew: FloatParam::new(
        "Skew",
        0.,
        3,
        FloatRange::Linear { min: -1., max: 1. }
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(2))
      .with_string_to_value(s2v_f32_percentage()),
    }
  }
}

impl PluginParameters for RepeatParameters {
  fn get_parameter(&self, index: i32) -> f32 {
    match index {
      0 => self.freq.get_normalized_value(),
      1 => self.repeats.get_normalized_value(),
      2 => self.feedback.get_normalized_value(),
      3 => self.skew.get_normalized_value(),
      _ => 0.0,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    match index {
      0 => self.freq.get_display_value(true),
      1 => self.repeats.get_display_value(true),
      2 => self.feedback.get_display_value(true),
      3 => self.skew.get_display_value(true),
      _ => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => self.freq.name,
      1 => self.repeats.name,
      2 => self.feedback.name,
      3 => self.skew.name,
      _ => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match index {
      0 => self.freq.set_plain_value(val),
      1 => self.repeats.set_normalized_value(val),
      2 => self.feedback.set_plain_value(val),
      3 => self.skew.set_plain_value(val),
      _ => (),
    }
  }
}
