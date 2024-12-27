#![feature(portable_simd)]
mod clipper;
mod op_amp;
mod params;
mod tone;
mod shared {
  pub mod float_ext;
  pub mod one_pole_filter;
}
pub use params::Params;
use {clipper::Clipper, op_amp::OpAmp, params::Smoother, tone::Tone};

pub struct Rat {
  op_amp: OpAmp,
  clipper: Clipper,
  tone: Tone,
}

impl Rat {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: OpAmp::new(sample_rate),
      clipper: Clipper::new(sample_rate),
      tone: Tone::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> f32 {
    let distortion = params.distortion.next();
    let filter = params.filter.next();
    let volume = params.volume.next();

    let op_amp_output = self.op_amp.process(input, distortion);
    let clipper_output = self.clipper.process(op_amp_output);
    let tone_output = self.tone.process(clipper_output, filter);
    tone_output * volume
  }
}
