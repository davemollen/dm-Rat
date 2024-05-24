extern crate lv2;
extern crate rat;
use lv2::prelude::*;
use rat::Rat;

#[derive(PortCollection)]
struct Ports {
  distortion: InputPort<Control>,
  filter: InputPort<Control>,
  volume: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Rat")]
struct DmRat {
  rat: Rat,
  is_active: bool,
}

impl Plugin for DmRat {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      rat: Rat::new(_plugin_info.sample_rate() as f32),
      is_active: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let distortion = *ports.distortion;
    let filter = *ports.filter;
    let volume = *ports.volume;
    let distortion = distortion * distortion;
    let filter = filter * filter;
    let volume = volume * volume;

    if !self.is_active {
      self.rat.initialize_params(distortion, filter, volume);
      self.is_active = true;
    }

    for (input, output) in ports.input.iter().zip(ports.output.iter_mut()) {
      *output = self.rat.process(*input, distortion, filter, volume);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmRat);
