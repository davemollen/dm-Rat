#[path = "./editor/components/param_knob.rs"]
mod param_knob;
use nih_plug::params::Param;
use param_knob::{ParamKnob, ParamKnobSize};
mod ui_data;
use crate::ds1_parameters::DS1Parameters;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::{
  model::Model,
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::Units::{Pixels, Stretch},
  style::FontWeightKeyword,
  views::{HStack, Label, VStack},
};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;
use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(|| (280, 160))
}

pub(crate) fn create(
  params: Arc<DS1Parameters>,
  editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(
    editor_state,
    ViziaTheming::Custom,
    move |cx, gui_context| {
      let _ = cx.add_stylesheet(STYLE);

      UiData {
        params: params.clone(),
        gui_context: gui_context.clone(),
      }
      .build(cx);

      VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.tone.name(),
            UiData::params,
            params.tone.as_ptr(),
            |params| &params.tone,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.level.name(),
            UiData::params,
            params.level.as_ptr(),
            |params| &params.level,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.dist.name(),
            UiData::params,
            params.dist.as_ptr(),
            |params| &params.dist,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(8.0));

        Label::new(cx, "DS1")
          .font_size(22.0)
          .font_weight(FontWeightKeyword::Bold)
          .border_radius(Pixels(16.0))
          .color("#eceaee")
          .background_color("#100f14")
          .child_space(Stretch(1.0))
          .child_top(Pixels(3.0))
          .child_bottom(Pixels(5.0))
          .width(Pixels(80.0))
          .left(Stretch(1.0));
      })
      .child_space(Pixels(16.0))
      .background_color("#DB783D");
    },
  )
}
