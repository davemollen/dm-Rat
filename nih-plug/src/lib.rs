use nih_plug::prelude::*;
use rat::{Params as ProcessParams, Rat};
use std::sync::Arc;
mod rat_parameters;
use rat_parameters::RatParameters;
mod editor;

struct DmRat {
  params: Arc<RatParameters>,
  rat: Rat,
  process_params: ProcessParams,
}

impl Default for DmRat {
  fn default() -> Self {
    let params = Arc::new(RatParameters::default());
    Self {
      params: params.clone(),
      rat: Rat::new(44100.),
      process_params: ProcessParams::new(44100.),
    }
  }
}

impl Plugin for DmRat {
  const NAME: &'static str = "Rat";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-Rat";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
    AudioIOLayout {
      main_input_channels: NonZeroU32::new(2),
      main_output_channels: NonZeroU32::new(2),
      ..AudioIOLayout::const_default()
    },
    AudioIOLayout {
      main_input_channels: NonZeroU32::new(1),
      main_output_channels: NonZeroU32::new(1),
      ..AudioIOLayout::const_default()
    },
  ];
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
    self.rat = Rat::new(buffer_config.sample_rate);
    self.process_params = ProcessParams::new(buffer_config.sample_rate);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    self.process_params.set(
      self.params.distortion.value(),
      self.params.filter.value(),
      self.params.volume.value(),
    );

    buffer.iter_samples().for_each(|mut channel_samples| {
      if channel_samples.len() == 2 {
        let channel_iterator = &mut channel_samples.iter_mut();
        let left_channel = channel_iterator.next().unwrap();
        let right_channel = channel_iterator.next().unwrap();
        let rat_out = self.rat.process(
          (*left_channel + *right_channel) * 0.5,
          &mut self.process_params,
        );
        *left_channel = rat_out;
        *right_channel = rat_out;
      } else {
        let sample = channel_samples.iter_mut().next().unwrap();
        *sample = self.rat.process(*sample, &mut self.process_params);
      };
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmRat {
  const CLAP_ID: &'static str = "dm-Rat";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A distortion plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Mono,
    ClapFeature::Distortion,
  ];
}

impl Vst3Plugin for DmRat {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-Rat..........";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx,
    Vst3SubCategory::Mono,
    Vst3SubCategory::Distortion,
  ];
}

nih_export_clap!(DmRat);
nih_export_vst3!(DmRat);
