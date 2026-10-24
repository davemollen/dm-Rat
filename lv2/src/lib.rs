extern crate lv2;
extern crate rat;
use lv2::prelude::*;
use rat::{Bypass, Params, Rat};

#[derive(PortCollection)]
struct Ports {
  distortion: InputPort<InPlaceControl>,
  filter: InputPort<InPlaceControl>,
  volume: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output: OutputPort<InPlaceAudio>,
  enabled: InputPort<InPlaceControl>,
}

#[uri("https://github.com/davemollen/dm-Rat")]
struct DmRat {
  rat: Rat,
  params: Params,
  bypass: Bypass,
}

impl Plugin for DmRat {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      rat: Rat::new(sample_rate),
      params: Params::new(sample_rate),
      bypass: Bypass::new(sample_rate),
    })
  }

  /// Reset the internal state of the plugin.
  fn activate(&mut self, _features: &mut Self::InitFeatures) {
    self.rat.reset();
    self.params.reset();
    self.bypass.reset();
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self.bypass.set_enabled(ports.enabled.get() == 0., || {
      self.rat.reset();
      self.params.reset();
    });
    self.params.set(
      ports.distortion.get(),
      ports.filter.get(),
      ports.volume.get(),
    );

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let dry = input.get();
      let rat_output = if self.bypass.is_enabled() {
        dry
      } else {
        let wet = self.rat.process(dry, &mut self.params);
        self.bypass.process(dry, wet)
      };
      output.set(rat_output);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmRat);
