use crate::repeat_parameters::{Params, RepeatParameters};
use std::sync::Arc;
use vizia::{
  prelude::{Lens, Wrapper, Event, EventContext}, state::Model
};
use vst::{host::Host, prelude::HostCallback};

fn notify_host_parameter_changed(index: i32, value: f32, host: Option<HostCallback>) {
  match host {
    Some(host) => {
      host.begin_edit(index);
      host.automate(index, value);
      host.end_edit(index);
    }
    None => {}
  }
}

pub enum ParamChangeEvent {
  SetFreq(f32),
  SetRepeats(f32),
  SetFeedback(f32),
  SetSkew(f32)
}

#[derive(Lens)]
pub struct UiData {
  pub params: Arc<RepeatParameters>,
  pub host: Option<HostCallback>,
}

impl Model for UiData {
  fn event(&mut self, _: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _| match app_event {
      ParamChangeEvent::SetFreq(value) => {
        let param = &self.params.freq;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetRepeats(value) => {
        let param = &self.params.repeats;
        param.set_normalized_value(*value);
        notify_host_parameter_changed(
          param.index,
          param.get_normalized_value(),
          self.host,
        );
      }

      ParamChangeEvent::SetFeedback(value) => {
        let param = &self.params.feedback;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }

      ParamChangeEvent::SetSkew(value) => {
        let param = &self.params.skew;
        param.set_plain_value(*value);
        notify_host_parameter_changed(param.index, *value, self.host);
      }
    });
  }
}
