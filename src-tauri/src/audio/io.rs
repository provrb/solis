use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, InputCallbackInfo, Stream, SupportedStreamConfig,
};
// use hound;
use rubato::{FftFixedIn, Resampler};
use std::collections::VecDeque;
use std::fs;
use std::sync::atomic::Ordering;
use std::{
    sync::{atomic::AtomicBool, Arc, LazyLock, Mutex},
    thread,
    time::Duration,
};
use whisper_rs::{
    FullParams, SamplingStrategy, SegmentCallbackData, WhisperContext, WhisperContextParameters,
};

static WHISPER_MODEL_PATH: &str =
    "/home/ethan/Repos/solis/src-tauri/src/audio/models/ggml-large-v3-turbo-q8_0.bin";
static WHISPER_PROMPT_PATH: &str = "../audio/models/prompt.txt";
pub static AUDIO_STREAMING_ACTIVE: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
pub struct AudioInput {
    pub whisper_context: WhisperContext,
    whisper_prompt: Option<String>,
    pub host_stream: Host,
    pub input_device: Device,
    pub input_stream: Option<Stream>,
    pub audio_buffer: Arc<Mutex<Vec<f32>>>,

    /// IO Volume
    /// Value between 0-100. When increasing the volume
    /// will be divided by 100 to get a 'gain factor.'
    /// When the audio samples are processed by CPAL, the
    /// will be multipled by the 'gain factor.'
    pub input_volume: i8,
    pub output_volume: i8,

    /// Input stream config
    pub stream_config: SupportedStreamConfig,
}

impl Default for AudioInput {
    fn default() -> Self {
        let model = WhisperContext::new_with_params(
            WHISPER_MODEL_PATH,
            WhisperContextParameters::default(),
        )
        .expect("error creating whisper medel");

        let host = cpal::default_host();
        let default_input_device = host
            .default_input_device()
            .expect("error getting default audio input device");
        let default_stream_config = default_input_device
            .default_input_config()
            .expect("error getting default stream config");

        Self {
            whisper_context: model,
            whisper_prompt: Self::get_whisper_initial_prompt(),
            input_device: default_input_device,
            stream_config: default_stream_config,
            host_stream: host,
            input_volume: 100,
            output_volume: 100,
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

    pub fn get_whisper_initial_prompt() -> Option<String> {
        fs::read_to_string(WHISPER_PROMPT_PATH).ok()
    }

    pub fn is_streaming_audio() -> bool {
        AUDIO_STREAMING_ACTIVE.load(Ordering::Relaxed)
    }

    pub fn set_output_volume(&mut self, new_volume: i8) {
        println!("setting output volume: {new_volume}");
        self.output_volume = new_volume;
    }

    pub fn set_input_volume(&mut self, new_volume: i8) {
        println!("setting output volume: {new_volume}");

        self.input_volume = new_volume;
    }

    pub fn set_input_device(&mut self, device_name: String) {
        if let Some(audio_device) = Self::get_audio_device_from_name(device_name) {
            self.input_device = audio_device;
        }
    }

    fn create_input_stream(&mut self) {
        self.audio_buffer = Arc::new(Mutex::new(Vec::new()));

        let config = self.stream_config.config();
        let buffer_clone = Arc::clone(&self.audio_buffer);
        let input_volume = self.input_volume as f32 / 100.0;
        let input_stream = self.input_device.build_input_stream(
            &config,
            move |data: &[f32], _: &InputCallbackInfo| {
                let mut buffer = buffer_clone.lock().unwrap();
                buffer.extend(data.iter().map(|sample| sample * input_volume));
            },
            move |err| eprintln!("input stream error, recording: {err}"),
            None,
        );

        match input_stream {
            Ok(s) => self.input_stream = Some(s),
            Err(e) => println!("error creating input stream: {e}"),
        }
    }

    pub fn stream_audio_input<F>(&mut self, stream_callback: F)
    where
        F: FnMut(SegmentCallbackData) + Send + 'static,
    {
        self.create_input_stream();
        AUDIO_STREAMING_ACTIVE.store(true, std::sync::atomic::Ordering::Relaxed);

        if let Some(stream) = &self.input_stream {
            stream.play().unwrap(); // start recording
        }

        let mut state = self.whisper_context.create_state().unwrap();
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_no_timestamps(true);
        params.set_no_context(true);
        params.set_segment_callback_safe(stream_callback);

        if let Some(initial_prompt) = &self.whisper_prompt {
            params.set_initial_prompt(initial_prompt);
        }

        let audio_buffer = Arc::clone(&self.audio_buffer);
        let mut last_pos = 0;

        let mut audio_queue: VecDeque<f32> = VecDeque::new();
        let max_queue_size = 16_000 * 30;

        while AUDIO_STREAMING_ACTIVE.load(Ordering::Relaxed) {
            if let Ok(buffer) = audio_buffer.try_lock() {
                for &sample in buffer[last_pos..].iter() {
                    audio_queue.push_back(sample);
                }
                last_pos = buffer.len();
            }

            while audio_queue.len() > max_queue_size {
                audio_queue.pop_front();
            }

            let chunk: Vec<f32> = audio_queue.iter().copied().collect();
            if chunk.is_empty() {
                thread::sleep(Duration::from_millis(10));
                continue;
            }
            let formatted = self.format_audio_samples(&chunk);

            if formatted.len() >= 16_000 * 2 {
                if !formatted.is_empty() && formatted.iter().any(|s| s.abs() > 0.05) {
                    state.full(params.clone(), &formatted).unwrap();
                }

                let overlap_samples = 16_000 / 4;
                let keep_samples = if audio_queue.len() > overlap_samples {
                    audio_queue.split_off(audio_queue.len() - overlap_samples)
                } else {
                    VecDeque::new()
                };
                audio_queue = keep_samples;
            }

            thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn end_streaming_input() {
        AUDIO_STREAMING_ACTIVE.store(false, Ordering::Relaxed);
    }

    pub fn transcribe_audio(&self, audio_buffer: Vec<f32>, params: Option<FullParams>) {
        let mut state = self
            .whisper_context
            .create_state()
            .expect("failed to create state");
        let params = if let Some(p) = params {
            p
        } else {
            FullParams::new(SamplingStrategy::Greedy { best_of: 1 })
        };

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

    fn format_audio_samples(&self, audio_buffer: &[f32]) -> Vec<f32> {
        let channels = self.stream_config.channels() as usize;
        let mono: Vec<f32> = if channels > 1 {
            audio_buffer
                .chunks(channels)
                .map(|frame| frame.iter().sum::<f32>() / channels as f32)
                .collect()
        } else {
            audio_buffer.to_vec()
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

        resampled_chunks
    }

    pub fn start_record_input(&mut self) {
        self.create_input_stream();
        println!("Starting to recrding input. Stream created.");

        if let Some(stream) = &self.input_stream {
            stream.play().unwrap();
        }
    }

    pub fn end_record_input(&mut self) -> Vec<f32> {
        if let Some(stream) = self.input_stream.take() {
            drop(stream);
        }

        let recorded = self.audio_buffer.lock().unwrap().clone();
        println!("Captured {} samples", recorded.len());
        self.format_audio_samples(&recorded)
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
