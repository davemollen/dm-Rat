use std::{sync::Arc, f32::consts::FRAC_1_SQRT_2};
use nih_plug::prelude::*;
use ds1::DS1;
mod ds1_parameters;
use ds1_parameters::DS1Parameters;
mod editor;

struct DmDS1 {
  params: Arc<DS1Parameters>,
  ds1: DS1,
}

impl Default for DmDS1 {
  fn default() -> Self {
    let params = Arc::new(DS1Parameters::default());
    Self {
      params: params.clone(),
      ds1: DS1::new(44100.),
    }
  }
}

impl Plugin for DmDS1 {
  const NAME: &'static str = "dm-DS1-fir";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-DS1";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  
  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(2),
    main_output_channels: NonZeroU32::new(2),
    ..AudioIOLayout::const_default()
  }];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;
  
  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.ds1 = DS1::new(buffer_config.sample_rate);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let tone = self.params.tone.value();
    let level = self.params.level.value();
    let dist = self.params.dist.value();

    buffer.iter_samples().for_each(|mut channel_samples| {
      let left_channel_in = channel_samples.get_mut(0).unwrap();
      let input_left = *left_channel_in;
      let right_channel_in = channel_samples.get_mut(1).unwrap();
      let input_right = *right_channel_in;

      let repeat_output = self.ds1.process(
        (input_left + input_right) * FRAC_1_SQRT_2,
        tone,
        level,
        dist
      );

      let left_channel_out = channel_samples.get_mut(0).unwrap();
      *left_channel_out = repeat_output;
      let right_channel_out = channel_samples.get_mut(1).unwrap();
      *right_channel_out = repeat_output;
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmDS1 {
  const CLAP_ID: &'static str = "dm-DS1-fir";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A distortion plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Mono,
    ClapFeature::Utility,
    ClapFeature::Distortion
  ];
}

impl Vst3Plugin for DmDS1 {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-DS1-fir......";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx, 
    Vst3SubCategory::Mono,
    Vst3SubCategory::Distortion
  ];
}

nih_export_clap!(DmDS1);
nih_export_vst3!(DmDS1);
