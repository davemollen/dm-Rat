#[macro_use]
extern crate vst;

use repeat::Repeat;
use std::sync::Arc;
use vst::{
  buffer::AudioBuffer,
  plugin::{Category, Info, Plugin, PluginParameters},
  util::AtomicFloat,
};

struct DmRepeat {
  params: Arc<RepeatParameters>,
  repeat: Repeat,
}

struct RepeatParameters {
  freq: AtomicFloat,
  repeats: AtomicFloat,
  feedback: AtomicFloat,
  skew: AtomicFloat,
}

impl Default for RepeatParameters {
  fn default() -> Self {
    Self {
      freq: AtomicFloat::new(4.0),
      repeats: AtomicFloat::new(4.0),
      feedback: AtomicFloat::new(1.0),
      skew: AtomicFloat::new(0.0),
    }
  }
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

impl PluginParameters for RepeatParameters {
  fn get_parameter(&self, index: i32) -> f32 {
    match index {
      0 => ((self.freq.get() - 0.1) / 49.9).powf(0.333333),
      1 => (self.repeats.get() - 1.0) / 31.0,
      2 => self.feedback.get() / 2.5 - 1.25,
      3 => self.skew.get() / 2. + 1.,
      _ => 0.0,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    match index {
      0 => format!("{:.2} hz", self.freq.get()),
      1 => format!("{:.2}", self.repeats.get()),
      2 => format!("{:.2}%", self.feedback.get() * 100.0),
      3 => format!("{:.2}%", self.skew.get() * 100.0),
      _ => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => "Frequency",
      1 => "Repeats",
      2 => "Feedback",
      3 => "Skew",
      _ => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match index {
      0 => self.freq.set(val.powf(3.) * 49.9 + 0.1),
      1 => self.repeats.set(val * 31.0 + 1.0),
      2 => self.feedback.set(val * 2.5 - 1.25),
      3 => self.skew.set(val * 2. - 1.),
      _ => (),
    }
  }
}

plugin_main!(DmRepeat);
