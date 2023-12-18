#[path="./ui/components/param_knob.rs"]
mod param_knob;
use nih_plug_vizia::vizia::{
  context::Context, 
  views::{VStack, HStack, Label}, 
  prelude::{Weight, Units::{Stretch, Pixels}, LayoutType}, 
  modifiers::{LayoutModifiers, TextModifiers, StyleModifiers}, state::Model
};
use param_knob::ParamKnob;
use crate::RepeatParameters;
use nih_plug::{prelude::GuiContext, params::Param};
use std::sync::Arc;
mod ui_data;
pub use ui_data::{UiData, ParamChangeEvent};

const STYLE: &str = include_str!("./ui/style.css");

pub fn plugin_gui(
  cx: &mut Context,
  params: Arc<RepeatParameters>,
  gui_context: Arc<dyn GuiContext>,
) {
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
      .background_color("#2d5f4f")
      .child_space(Stretch(1.0))
      .child_top(Pixels(1.0))
      .child_bottom(Pixels(5.0))
      .width(Pixels(144.0))
      .left(Stretch(1.0));
  })
  .layout_type(LayoutType::Column)
  .child_space(Pixels(16.0))
  .background_color("#161616");
}