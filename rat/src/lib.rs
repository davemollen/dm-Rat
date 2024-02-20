#![feature(portable_simd)]
mod op_amp;
use op_amp::OpAmp;
mod tone;
use tone::Tone;
mod clipper;
use clipper::Clipper;
mod smooth_parameters;
use smooth_parameters::SmoothParameters;
mod shared {
  pub mod lowpass_filter;
}

pub struct Rat {
  op_amp: OpAmp,
  clipper: Clipper,
  tone: Tone,
  smooth_parameters: SmoothParameters,
}

impl Rat {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: OpAmp::new(sample_rate),
      clipper: Clipper::new(),
      tone: Tone::new(sample_rate),
      smooth_parameters: SmoothParameters::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, distortion: f32, filter: f32, volume: f32) -> f32 {
    let (distortion, filter, volume) =
      self
        .smooth_parameters
        .process(distortion * distortion, filter * filter, volume * volume);
    let op_amp_output = self.op_amp.process(input, distortion);
    let clipper_output = self.clipper.process(op_amp_output);
    let tone_output = self.tone.process(clipper_output, filter);
    tone_output * volume
  }
}
