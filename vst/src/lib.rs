#[macro_use]
extern crate vst;
mod repeat_parameters;
use repeat::Repeat;
use repeat_parameters::RepeatParameters;
use std::sync::Arc;
use vst::{
  buffer::AudioBuffer,
  plugin::{Category, Info, Plugin, PluginParameters},
};

struct DmRepeat {
  params: Arc<RepeatParameters>,
  repeat: Repeat,
}

impl Default for DmRepeat {
  fn default() -> Self {
    Self {
      params: Arc::new(RepeatParameters::default()),
      repeat: Repeat::new(44100.),
    }
  }
}

impl Plugin for DmRepeat {
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
    let freq = self.params.freq.get();
    let repeats = self.params.repeats.get();
    let feedback = self.params.feedback.get();
    let skew = self.params.skew.get();

    for (input_buffer, output_buffer) in buffer.zip() {
      for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
        *output_sample = self
          .repeat
          .run(*input_sample, freq, repeats, feedback, skew);
      }
    }
  }

  fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
    Arc::clone(&self.params) as Arc<dyn PluginParameters>
  }
}

plugin_main!(DmRepeat);
