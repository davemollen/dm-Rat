mod repeat_parameters;
use repeat_parameters::RepeatParameters;
use std::sync::Arc;
use vizia::prelude::*;
mod editor;
use editor::{plugin_gui, WINDOW_SIZE};

fn main() {
  let params = Arc::new(RepeatParameters::default());

  Application::new(move |cx| plugin_gui(cx, Arc::clone(&params), None))
    .title("dm-Repeat")
    .inner_size((WINDOW_SIZE.width, WINDOW_SIZE.height))
    .run();
}
