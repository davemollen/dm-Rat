#![feature(portable_simd)]
mod transistor_booster;
use transistor_booster::TransistorBooster;
mod op_amp;
use op_amp::OpAmp;
mod clipper;
use clipper::Clipper;
mod tone;
use tone::Tone;
pub mod shared {
  pub mod lowpass_filter;
}

pub struct DS1 {
  transistor_booster: TransistorBooster,
  op_amp: OpAmp,
  clipper: Clipper,
  tone: Tone,
}

impl DS1 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      transistor_booster: TransistorBooster::new(sample_rate),
      op_amp: OpAmp::new(sample_rate),
      clipper: Clipper::new(sample_rate),
      tone: Tone::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, tone: f32, level: f32, dist: f32) -> f32 {
    let booster_output = self.transistor_booster.process(input, 63.095734);
    let op_amp_output = self.op_amp.process(booster_output, dist);
    let clip_output = self.clipper.process(op_amp_output);
    let tone_output = self.tone.process(clip_output, tone);

    tone_output * level
  }
}
