use std::sync::Arc;

use nih_plug::{
  formatters::{s2v_f32_percentage, v2s_f32_percentage},
  prelude::{IntParam, IntRange, FloatParam, FloatRange, Params}
};
mod custom_formatters;
use custom_formatters::v2s_f32_digits;
use nih_plug_vizia::ViziaState;

use crate::editor;

#[derive(Params)]
pub struct RepeatParameters {
  /// The editor state, saved together with the parameter state so the custom scaling can be
  /// restored.
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "freq"]
  pub freq: FloatParam,

  #[id = "repeats"]
  pub repeats: IntParam,

  #[id = "feedback"]
  pub feedback: FloatParam,

  #[id = "skew"]
  pub skew: FloatParam,
}

impl Default for RepeatParameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      freq: FloatParam::new(
        "Freq",
        4.,
        FloatRange::Skewed {
          min: 0.1,
          max: 50.,
          factor: 0.3
        },
      )
      .with_unit(" Hz")
      .with_value_to_string(v2s_f32_digits(2)),

      repeats: IntParam::new(
        "Repeats",
        4,
        IntRange::Linear {
          min: 1,
          max: 24
        }
      )
      .with_unit(" x"),

      feedback: FloatParam::new(
        "Feedback",
        1.,
        FloatRange::Linear { 
          min: -1.25, 
          max: 1.25 
        },
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(2))
      .with_string_to_value(s2v_f32_percentage()),

      skew: FloatParam::new(
        "Skew",
        0.,
        FloatRange::Linear {
          min: -1.,
          max: 1.
        }
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(2))
      .with_string_to_value(s2v_f32_percentage()),
    }
  }
}
