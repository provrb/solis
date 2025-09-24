use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, InputCallbackInfo, Stream, SupportedStreamConfig,
};
use rubato::{FftFixedIn, Resampler};
use std::sync::{Arc, Mutex};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

static WHISPER_MODEL_PATH: &str =
    "/home/ethan/Repos/solis/src-tauri/src/audio/models/ggml-base-q8_0.bin";

pub struct AudioInput {
    pub whisper_context: WhisperContext,
    pub host_stream: Host,
    pub input_device: Device,
    pub input_stream: Option<Stream>,
    pub audio_buffer: Arc<Mutex<Vec<f32>>>,

    /// Input stream config
    pub stream_config: SupportedStreamConfig,
}

impl Default for AudioInput {
    fn default() -> Self {
        let model = WhisperContext::new_with_params(
            WHISPER_MODEL_PATH,
            WhisperContextParameters::default(),
        )
        .expect("error creating whisper model");

        let host = cpal::default_host();
        let default_input_device = host
            .default_input_device()
            .expect("error getting default audio input device");
        let default_stream_config = default_input_device
            .default_input_config()
            .expect("error getting default stream config");

        Self {
            whisper_context: model,
            input_device: default_input_device,
            stream_config: default_stream_config,
            host_stream: host,
            input_stream: None,
            audio_buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl AudioInput {
    pub fn get_audio_input_devices() -> Vec<String> {
        let host = cpal::default_host();
        let Ok(input_devices) = host.input_devices() else {
            return Vec::new();
        };

        input_devices
            .into_iter()
            .filter_map(|device| device.name().ok())
            .collect()
    }

    pub fn get_audio_output_devices() -> Vec<String> {
        let host = cpal::default_host();
        let Ok(output_devices) = host.output_devices() else {
            return Vec::new();
        };

        output_devices
            .into_iter()
            .filter_map(|device| device.name().ok())
            .collect()
    }

    fn create_input_stream(&mut self) {
        let config = self.stream_config.config();
        self.audio_buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = Arc::clone(&self.audio_buffer);

        let input_stream = self.input_device.build_input_stream(
            &config,
            move |data: &[f32], _: &InputCallbackInfo| {
                let mut buffer = buffer_clone.lock().unwrap();
                buffer.extend_from_slice(data);
            },
            move |err| eprintln!("input stream error, recording: {err}"),
            None,
        );

        match input_stream {
            Ok(s) => self.input_stream = Some(s),
            Err(e) => println!("error creating input stream: {e}"),
        }
    }

    pub fn start_record_input(&mut self) {
        self.create_input_stream();
        println!("Starting to recrding input. Stream created.");

        if let Some(stream) = &self.input_stream {
            stream.play().unwrap();
        }
    }

    pub fn transcribe_audio(&self, audio_buffer: Vec<f32>) {
        let mut state = self
            .whisper_context
            .create_state()
            .expect("failed to create state");
        let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        state
            .full(params, &audio_buffer)
            .expect("failed to run model");

        let num_segments = state.full_n_segments();
        for i in 0..num_segments {
            let segment = state.get_segment(i).expect("failed to get segment {i}");
            let text = segment
                .to_str()
                .expect("failed retrieving text from segment");
            println!("Segment: {i}, {text}");
        }
    }

    pub fn end_record_input(&mut self) -> Vec<f32> {
        if let Some(stream) = self.input_stream.take() {
            drop(stream);
        }

        let recorded = self.audio_buffer.lock().unwrap().clone();
        println!("Captured {} samples", recorded.len());

        let channels = self.stream_config.channels() as usize;
        let mono: Vec<f32> = if channels > 1 {
            recorded
                .chunks(channels)
                .map(|frame| frame.iter().sum::<f32>() / channels as f32)
                .collect()
        } else {
            recorded
        };

        let in_rate = self.stream_config.sample_rate().0 as usize;
        const OUT_RATE: usize = 16000;
        const CHUNK_SIZE: usize = 1024;

        let mut resampler = FftFixedIn::<f32>::new(in_rate, OUT_RATE, CHUNK_SIZE, 2, 1).unwrap();
        let mut resampled_chunks = Vec::new();
        for chunk in mono.chunks(CHUNK_SIZE) {
            let mut padded = chunk.to_vec();
            if padded.len() < CHUNK_SIZE {
                padded.resize(CHUNK_SIZE, 0.0); // pad with silence
            }
            let processed = resampler.process(&[padded], None).unwrap();
            resampled_chunks.extend(processed.concat());
        }

        println!(
            "Resampled to {} Hz, {} samples",
            OUT_RATE,
            resampled_chunks.len()
        );
        resampled_chunks
    }

    fn get_audio_device_from_name(device_name: String) -> Option<Device> {
        let host = cpal::default_host();
        let devices = host
            .input_devices()
            .into_iter()
            .flatten()
            .chain(host.output_devices().into_iter().flatten());

        for device in devices {
            if let Ok(name) = device.name() {
                if name == device_name {
                    return Some(device);
                }
            }
        }

        None
    }
}
