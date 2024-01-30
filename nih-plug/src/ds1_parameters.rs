use std::sync::Arc;
use nih_plug::{
  formatters::{s2v_f32_percentage, v2s_f32_percentage},
  prelude::{FloatParam, FloatRange, Params}
};
use nih_plug_vizia::ViziaState;

use crate::editor;

#[derive(Params)]
pub struct DS1Parameters {
  /// The editor state, saved together with the parameter state so the custom scaling can be
  /// restored.
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "tone"]
  pub tone: FloatParam,

  #[id = "level"]
  pub level: FloatParam,

  #[id = "dist"]
  pub dist: FloatParam,
}

impl Default for DS1Parameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      tone: FloatParam::new(
        "Tone",
        0.5,
        FloatRange::Linear {
          min: 0.,
          max: 1.
        },
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(2))
      .with_string_to_value(s2v_f32_percentage()),

      level: FloatParam::new(
        "Level",
        0.5,
        FloatRange::Linear {
          min: 0.,
          max: 1.
        }
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(2))
      .with_string_to_value(s2v_f32_percentage()),

      dist: FloatParam::new(
        "Dist",
        0.5,
        FloatRange::Linear { 
          min: 0., 
          max: 1.
        },
      )
      .with_unit(" %")
      .with_value_to_string(v2s_f32_percentage(2))
      .with_string_to_value(s2v_f32_percentage()),
    }
  }
}
