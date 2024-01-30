mod highpass_filter;
use highpass_filter::HighpassFilter;
use crate::shared::lowpass_filter::LowpassFilter;

pub struct Tone {
  lowpass: LowpassFilter,
  highpass: HighpassFilter
}

impl Tone {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass: LowpassFilter::new(sample_rate),
      highpass: HighpassFilter::new(sample_rate)
    }
  }

  pub fn process(&mut self, input: f32, tone: f32) -> f32 {
    let lowpass_output = self.lowpass.process(input, 234.05138689985) * ((1. - tone) * 0.595235 + 0.202379);
    let highpass_output = self.highpass.process(input, 1063.8699404538) * (tone * 0.694642 + 0.002896);
    
 	  lowpass_output + highpass_output
  }
}