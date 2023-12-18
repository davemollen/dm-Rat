use nih_plug::prelude::Editor;
use nih_plug_vizia::{ViziaState, ViziaTheming, create_vizia_editor, widgets::ResizeHandle};
use std::sync::Arc;
use crate::repeat_parameters::RepeatParameters;
mod ui;
use ui::plugin_gui;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (360, 200))
}

pub(crate) fn create(
    params: Arc<RepeatParameters>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, gui_context| { 
      plugin_gui(cx, params.clone(), gui_context);
      ResizeHandle::new(cx);
    })
}
