#[macro_use]
extern crate vst;

use repeat::Repeat;
use std::sync::Arc;
use vst::api::TimeInfo;
use vst::buffer::AudioBuffer;
use vst::host::Host;
use vst::plugin::{HostCallback, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

struct DmRepeat {
    params: Arc<RepeatParameters>,
    repeat: Repeat,
}

struct RepeatParameters {
    freq: AtomicFloat,
    repeats: AtomicFloat,
    feedback: AtomicFloat,
    mix: AtomicFloat,
}

impl Default for RepeatParameters {
    fn default() -> Self {
        Self {
            freq: AtomicFloat::new(2.0),
            repeats: AtomicFloat::new(7.0),
            feedback: AtomicFloat::new(0.),
            mix: AtomicFloat::new(0.5),
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
    fn new(host: HostCallback) -> Self {
        fn get_sample_rate(info: TimeInfo) -> f64 {
            info.sample_rate
        }
        let sample_rate = host.get_time_info(0).map(get_sample_rate).unwrap();
        Self {
            params: Arc::new(RepeatParameters::default()),
            repeat: Repeat::new(sample_rate),
        }
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.repeat = Repeat::new(f64::from(sample_rate));
    }

    fn get_info(&self) -> Info {
        Info {
            name: "dm-Repeat".to_string(),
            inputs: 1,
            outputs: 1,
            parameters: 8,
            unique_id: 1358,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let freq = self.params.freq.get();
        let repeats = self.params.repeats.get();
        let feedback = self.params.feedback.get();
        let mix = self.params.mix.get();

        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                *output_sample = self.repeat.run(
                    *input_sample,
                    freq,
                    repeats,
                    feedback,
                    mix,
                );
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
            0 => ((self.freq.get() - 0.2) / 48.8).powf(0.333333),
            1 => ((self.repeats.get() + 1.0) * 16.0).floor(),
            2 => self.feedback.get(),
            3 => self.mix.get(),
            _ => 0.0,
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2} hz", self.freq.get()),
            1 => format!("{} ", self.repeats.get()),
            2 => format!("{:.2} st", self.feedback.get() * 100.0),
            3 => format!("{:.2}%", self.mix.get() * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Frequency",
            1 => "Repeats",
            2 => "Feedback",
            3 => "Mix",
            _ => "",
        }
        .to_string()
    }

    fn set_parameter(&self, index: i32, val: f32) {
        match index {
            0 => self.freq.set(val.powf(3.) * 48.8 + 0.2),
            1 => self.repeats.set(val * 16.0 - 1.0),
            2 => self.feedback.set(val),
            3 => self.mix.set(val),
            _ => (),
        }
    }
}

plugin_main!(DmRepeat);
