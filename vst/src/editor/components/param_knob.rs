use crate::repeat_parameters::{FloatParam, Params};
use std::any::Any;
use vizia::{
  prelude::{ActionModifiers, Context, EmitContext, LensExt, LayoutModifiers, Units::{Stretch, Pixels}, Weight},
  views::{Knob, Label, TextEvent, Textbox, VStack}, 
  modifiers::{TextModifiers, StyleModifiers}, 
  handle::Handle, 
  state::Lens,
};

pub struct ParamKnob;

impl ParamKnob {
  pub fn new<'a, L, F, M, C>(
    cx: &'a mut Context,
    name: &'a str,
    lens: L,
    params_to_param: F,
    on_change: C,
  ) -> Handle<'a, VStack> 
  where
    L: 'static + Lens + Copy + Send + Sync,
    <L as Lens>::Source: 'static,
    F: 'static + Fn(&<L as Lens>::Target) -> &FloatParam + Copy + Send + Sync,
    M: Any + Send,
    C: 'static + Fn(f32) -> M + Copy + Send + Sync,
  {
    VStack::new(cx, |cx| {
      Label::new(cx, name)
        .font_size(13.0)
        .font_weight(Weight::BOLD)
        .text_wrap(false)
        .child_space(Stretch(1.0));

      Knob::new(
        cx,
        lens.map(move |p| params_to_param(p).get_default_normalized_value()),
        lens.map(move |p| params_to_param(p).get_normalized_value()),
        false,
      )
      .on_changing(move |cx, val| {
        cx.emit(on_change(val));
      });

      Textbox::new(
        cx,
        lens.map(move |p| params_to_param(p).get_display_value(true)),
      )
      .on_mouse_down(|cx, _| {
        cx.emit(TextEvent::StartEdit);
        cx.emit(TextEvent::ResetText("".to_string()));
      })
      .on_submit(move |cx, text, success| {
        cx.emit(TextEvent::EndEdit);

        if success {
          let normalized_value =
            lens.map(move |p| params_to_param(p).string_to_normalized_value(&text));
          match normalized_value.get(cx) {
            Some(val) => cx.emit(on_change(val)),
            _ => (),
          };
        }
      })
      .font_size(12.0)
      .class("align_center");
    })
    .child_space(Stretch(1.0))
    .row_between(Pixels(4.0))
  }
}
