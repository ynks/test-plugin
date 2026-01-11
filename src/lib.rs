use nih_plug::prelude::*;
use std::sync::Arc;


struct TestPlugin {
	params: Arc<TestPluginParams>,
}

#[derive(Params)]
struct TestPluginParams {
	#[id = "gain"]
	pub gain: FloatParam,
}

impl Default for TestPlugin {
	fn default() -> Self {
		Self {
			params: Arc::new(TestPluginParams::default()),
		}
	}
}

impl Default for TestPluginParams {
	fn default() -> Self {
		Self {
			gain: FloatParam::new(
				"Gain",
				util::db_to_gain(0.0),
				FloatRange::Skewed {
					min: util::db_to_gain(-30.0),
					max: util::db_to_gain(30.0),
					factor: FloatRange::gain_skew_factor(-30.0, 30.0),
				},
			)
			.with_smoother(SmoothingStyle::Logarithmic(50.0))
			.with_unit(" dB")
			.with_value_to_string(formatters::v2s_f32_gain_to_db(2))
			.with_string_to_value(formatters::s2v_f32_gain_to_db()),
		}
	}
}

impl Plugin for TestPlugin {
	const NAME: &'static str = "Test Plugin";
	const VENDOR: &'static str = "Xein";
	const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
	const EMAIL: &'static str = "xgonip@gmail.com";

	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	// The first audio IO layout is used as the default
	const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
		main_input_channels: NonZeroU32::new(2),
		main_output_channels: NonZeroU32::new(2),

		aux_input_ports: &[],
		aux_output_ports: &[],

		names: PortNames::const_default(),
	}];


	const MIDI_INPUT: MidiConfig = MidiConfig::None;
	const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

	const SAMPLE_ACCURATE_AUTOMATION: bool = true;

	type SysExMessage = ();
	type BackgroundTask = ();

	fn params(&self) -> Arc<dyn Params> {
		self.params.clone()
	}

	fn initialize(
		&mut self,
		_audio_io_layout: &AudioIOLayout,
		_buffer_config: &BufferConfig,
		_context: &mut impl InitContext<Self>,
	) -> bool {
		// Resize buffers and perform other potentially expensive initialization operations here
		true
	}

	fn reset(&mut self) {
		// Reset buffers and envelopes here
	}

	fn process(
		&mut self,
		buffer: &mut Buffer,
		_aux: &mut AuxiliaryBuffers,
		_context: &mut impl ProcessContext<Self>,
	) -> ProcessStatus {
		for channel_samples in buffer.iter_samples() {
			let gain = self.params.gain.smoothed.next();

			for sample in channel_samples {
				*sample *= gain;
			}
		}

		ProcessStatus::Normal
	}
}

impl ClapPlugin for TestPlugin {
	const CLAP_ID: &'static str = "com.xein.testplugin";
	const CLAP_DESCRIPTION: Option<&'static str> = Some("A simple test plugin to try building a VST in rust");
	const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
	const CLAP_SUPPORT_URL: Option<&'static str> = None;
	const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for TestPlugin {
	const VST3_CLASS_ID: [u8; 16] = *b"XEIN-20260111-01";
	const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(TestPlugin);
nih_export_vst3!(TestPlugin);
