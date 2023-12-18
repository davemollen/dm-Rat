#[macro_use]
extern crate vst;
mod editor;
use editor::RepeatEditor;
mod repeat_parameters;
use repeat::Repeat;
use repeat_parameters::{RepeatParameters, Params};
use std::sync::Arc;
use vst::{
  buffer::AudioBuffer,
  plugin::{Category, Info, Plugin, PluginParameters},
  prelude::HostCallback, editor::Editor,
};

struct DmRepeat {
  params: Arc<RepeatParameters>,
  repeat: Repeat,
  editor: Option<RepeatEditor>,
}

impl Plugin for DmRepeat {
  fn new(host: HostCallback) -> Self {
    let params = Arc::new(RepeatParameters::default());

    Self {
      params: params.clone(),
      repeat: Repeat::new(44100.),
      editor: Some(RepeatEditor {
        params: params.clone(),
        is_open: false,
        host: Some(host),
      })
    }
  }

  fn set_sample_rate(&mut self, sample_rate: f32) {
    self.repeat = Repeat::new(sample_rate);
  }

  fn get_info(&self) -> Info {
    Info {
      name: "dm-Repeat".to_string(),
      vendor: "DM".to_string(),
      version: 1,
      inputs: 1,
      outputs: 1,
      parameters: 4,
      unique_id: 1357,
      f64_precision: true,
      category: Category::Effect,
      ..Default::default()
    }
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    let freq = self.params.freq.get_value();
    let repeats = self.params.repeats.get_value();
    let feedback = self.params.feedback.get_value();
    let skew = self.params.skew.get_value();

    for (input_buffer, output_buffer) in buffer.zip() {
      for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
        *output_sample = self.repeat.run(
          *input_sample,
          freq,
          repeats as usize,
          feedback,
          skew,
        );
      }
    }
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.params) as Arc<dyn PluginParameters>
  }

  fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
    if let Some(editor) = self.editor.take() {
      Some(Box::new(editor) as Box<dyn Editor>)
    } else {
      None
    }
  }
}

plugin_main!(DmRepeat);
