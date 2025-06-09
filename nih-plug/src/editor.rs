#[path = "./editor/components/param_knob.rs"]
mod param_knob;
use nih_plug::params::Param;
use param_knob::{ParamKnob, ParamKnobSize};
mod ui_data;
use crate::rat_parameters::RatParameters;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::{
  model::Model,
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::Units::{Pixels, Stretch},
  style::FontWeightKeyword,
  views::{HStack, Label, VStack},
};
use nih_plug_vizia::{create_vizia_editor, vizia_assets, ViziaState, ViziaTheming};
use std::sync::Arc;
use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(|| (280, 200))
}

pub(crate) fn create(
  params: Arc<RatParameters>,
  editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(
    editor_state,
    ViziaTheming::Custom,
    move |cx, gui_context| {
      vizia_assets::register_roboto(cx);
      vizia_assets::register_roboto_bold(cx);
      cx.add_stylesheet(STYLE).ok();

      UiData {
        params: params.clone(),
        gui_context: gui_context.clone(),
      }
      .build(cx);

      VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.distortion.name(),
            UiData::params,
            params.distortion.as_ptr(),
            |params| &params.distortion,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.filter.name(),
            UiData::params,
            params.filter.as_ptr(),
            |params| &params.filter,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.volume.name(),
            UiData::params,
            params.volume.as_ptr(),
            |params| &params.volume,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(8.0));

        Label::new(cx, "RAT")
          .font_size(32.0)
          .font_weight(FontWeightKeyword::ExtraBold)
          .color("#eceaee")
          .background_color("#100f14")
          .border_color("#eceaee")
          .border_width(Pixels(1.0))
          .child_space(Stretch(1.0))
          .width(Pixels(66.0))
          .height(Pixels(32.0))
          .top(Pixels(32.0))
          .bottom(Pixels(32.0))
          .left(Stretch(1.0))
          .right(Stretch(1.0));
      })
      .child_space(Pixels(16.0))
      .background_color("#161616");
    },
  )
}
