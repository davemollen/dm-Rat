#[path="./editor/components/param_knob.rs"]
mod param_knob;
use param_knob::ParamKnob;
use nih_plug::params::Param;
mod ui_data;
use ui_data::{UiData, ParamChangeEvent};
use nih_plug::prelude::Editor;
use nih_plug_vizia::{ViziaState, ViziaTheming, create_vizia_editor};
use nih_plug_vizia::vizia::{
  views::{VStack, HStack, Label}, 
  prelude::{Weight, Units::{Stretch, Pixels}, LayoutType}, 
  modifiers::{LayoutModifiers, TextModifiers, StyleModifiers}, state::Model
};
use std::sync::Arc;
use crate::repeat_parameters::RepeatParameters;

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (360, 200))
}

pub(crate) fn create(
    params: Arc<RepeatParameters>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, gui_context| { 
      cx.add_theme(STYLE);
      
      UiData {
        params: params.clone(),
        gui_context: gui_context.clone()
      }.build(cx);

      VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.freq.name(),
            UiData::params,
            params.freq.as_ptr(),
            |params| &params.freq,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          );
          
          ParamKnob::new(
            cx,
            params.repeats.name(),
            UiData::params,
            params.repeats.as_ptr(),
            |params| &params.repeats,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          );
          
          ParamKnob::new(
            cx,
            params.feedback.name(),
            UiData::params,
            params.feedback.as_ptr(),
            |params| &params.feedback,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          );
          
          ParamKnob::new(
            cx,
            params.skew.name(),
            UiData::params,
            params.skew.as_ptr(),
            |params| &params.skew,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
          )
          .bottom(Pixels(24.0));
        }).top(Pixels(16.0));
        Label::new(cx, "dm-Repeat")
          .font_size(22.0)
          .font_weight(Weight::BOLD)
          .border_radius(Pixels(16.0))
          .border_width(Pixels(2.))
          .border_color("#005254")
          .background_color("#009092")
          .child_space(Stretch(1.0))
          .child_top(Pixels(1.0))
          .child_bottom(Pixels(5.0))
          .width(Pixels(144.0))
          .left(Stretch(1.0));
      })
      .layout_type(LayoutType::Column)
      .child_space(Pixels(16.0))
      .background_color("#161616");
    })
}
