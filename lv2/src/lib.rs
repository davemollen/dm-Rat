extern crate repeat;
extern crate lv2;
use repeat::Repeat;
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
    freq: InputPort<Control>,
    repeats: InputPort<Control>,
    feedback: InputPort<Control>,
    mix: InputPort<Control>,
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Repeat")]
struct DmRepeat {
    repeat: Repeat,
}

impl Plugin for DmRepeat {
    // Tell the framework which ports this plugin has.
    type Ports = Ports;

    // We don't need any special host features; We can leave them out.
    type InitFeatures = ();
    type AudioFeatures = ();

    // Create a new instance of the plugin; Trivial in this case.
    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self {
            repeat: Repeat::new(_plugin_info.sample_rate()),
        })
    }

    // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
    // iterates over.
    fn run(&mut self, ports: &mut Ports, _features: &mut ()) {
        let frequency = *ports.freq;
        let repeats = *ports.repeats;
        let feedback = *ports.feedback * 0.01;
        let mix = *ports.mix * 0.01;

        for (in_frame, out_frame) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            *out_frame = self.repeat.run(
                *in_frame, frequency, repeats, feedback, mix,
            );
        }
    }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmRepeat);
