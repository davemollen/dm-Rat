#[path="./components/param_knob.rs"]
mod param_knob;
use param_knob::ParamKnob;
#[path="./components/param_int_knob.rs"]
mod param_int_knob;
use param_int_knob::ParamIntKnob;
#[path="ui_data.rs"]
mod ui_data;
use ui_data::{UiData, ParamChangeEvent};
use vizia::{
  views::{VStack, HStack, Label}, 
  context::Context, 
  prelude::{Units::{Stretch, Pixels}, Weight, LayoutType}, 
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers}, 
  state::Model
};
use crate::repeat_parameters::RepeatParameters;
use std::sync::Arc;
use vst::prelude::HostCallback;

const STYLE: &str = include_str!("./style.css");

pub fn plugin_gui(cx: &mut Context, params: Arc<RepeatParameters>, host: Option<HostCallback>) {
  cx.add_theme(STYLE);

  UiData {
    params: params.clone(),
    host,
  }.build(cx);
  
  VStack::new(cx, |cx| {
    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        params.freq.name,
        UiData::params,
        |params| &params.freq,
        |val| ParamChangeEvent::SetFreq(val),
      );
      
      ParamIntKnob::new(
        cx,
        params.repeats.name,
        UiData::params,
        |params| &params.repeats,
        |val| ParamChangeEvent::SetRepeats(val),
      );
      
      ParamKnob::new(
        cx,
        params.feedback.name,
        UiData::params,
        |params| &params.feedback,
        |val| ParamChangeEvent::SetFeedback(val),
      );
      
      ParamKnob::new(
        cx,
        params.skew.name,
        UiData::params,
        |params| &params.skew,
        |val| ParamChangeEvent::SetSkew(val),
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