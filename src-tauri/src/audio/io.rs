use std::{ffi::CStr, sync::{Arc, Mutex}};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait}, Device, Host, InputCallbackInfo, Stream, SupportedStreamConfig
};

static WHISPER_MODEL_PATH: &str = "models/ggml-base-q8_0.bin";

pub struct AudioInput {
    pub whisper_context: WhisperContext,
    pub host_stream: Host,
    pub input_device: Device,
    pub input_stream: Option<Stream>,
    pub audio_buffer: Arc<Mutex<Vec<f32>>>,

    /// Input stream config
    pub stream_config: SupportedStreamConfig, 
}

impl AudioInput {
    pub fn new() -> Self {
        let host = cpal::default_host();
        
        let model = match WhisperContext::new_with_params(
            WHISPER_MODEL_PATH,
            WhisperContextParameters::default(),
        ) {
            Ok(model) => model,
            Err(e) => panic!("error creating whisper model: {e}"),
        };

        let default_input_device = match host.default_input_device() {
            Some(d) => d,
            None => panic!("failed getting default audio input device"),
        };
        
        let default_stream_config = match default_input_device.default_input_config() {
            Ok(d) => d,
            Err(e) => panic!("failed getting default stream config: {e}"),
        };

        Self {
            whisper_context: model,
            input_device: default_input_device,
            stream_config: default_stream_config,
            host_stream: host,
            input_stream: None,
            audio_buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_audio_input_devices() -> Vec<String> {
        let host = cpal::default_host();
        let Ok(input_devices) = host.input_devices() else {
            return Vec::new();
        };

        input_devices
            .into_iter()
            .filter(|device| device.name().is_ok())
            .map(|device| device.name().unwrap())
            .collect()
    }

    pub fn get_audio_output_devices() -> Vec<String> {
        let host = cpal::default_host();
        let Ok(output_devices) = host.output_devices() else {
            return Vec::new();
        };

        output_devices
            .into_iter()
            .filter(|device| device.name().is_ok())
            .map(|device| device.name().unwrap())
            .collect()
    }

    /// not mine
    /// https://github.com/lmammino/whisper-rs-example/blob/main/src/main.rs
    extern "C" fn whisper_on_segment(
        _ctx: *mut whisper_rs_sys::whisper_context,
        state: *mut whisper_rs_sys::whisper_state,
        _n_new: std::os::raw::c_int,
        _user_data: *mut std::os::raw::c_void,
    ) {
        let last_segment = unsafe { whisper_rs_sys::whisper_full_n_segments_from_state(state) } - 1;
        let ret =
            unsafe { whisper_rs_sys::whisper_full_get_segment_text_from_state(state, last_segment) };
        if ret.is_null() {
            panic!("Failed to get segment text")
        }
        let c_str = unsafe { CStr::from_ptr(ret) };
        let r_str = c_str.to_str().expect("invalid segment text");
        println!("-> Segment ({}) text: {}", last_segment, r_str)
    }

    fn create_input_stream(&mut self) {
        let config = self.stream_config.config();
        self.audio_buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = Arc::clone(&self.audio_buffer);

        let input_stream = self.input_device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &InputCallbackInfo| {
                let mut buffer = buffer_clone.lock().unwrap();
                buffer.extend_from_slice(data);
            },
            move |err| {
                eprintln!("input stream error, recording: {err}")
            },
            None
        );

        match input_stream {
            Ok(s) => self.input_stream = Some(s),
            Err(e) => println!("error creating input stream: {e}"),
        }
    }

    pub fn start_record_input(&mut self) {
        self.create_input_stream();

        if let Some(stream) = &self.input_stream {
            let _ = stream.play();
        }
    }

    pub fn end_record_input(&mut self) {
        if let Some(stream) = self.input_stream.take() {
            // Dropping stream stops the recording
            drop(stream);
        }

        let audio: Vec<f32> = {
            let mut locked = self.audio_buffer.lock().unwrap();
            let audio = locked.clone();
            locked.clear();
            audio
        };

        println!("# of audio samples: {}", audio.len());

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_print_progress(true);
        params.set_language(Some("en"));
        unsafe {
            params.set_new_segment_callback(Some(Self::whisper_on_segment)); // Static method
        }
        params.set_progress_callback_safe(|progress| println!("Progress: {}", progress));

        let st = std::time::Instant::now();

        let mut state = self.whisper_context.create_state().expect("failed to create state");

        state.full(params, &audio[..]).expect("failed to run model");

        let num_segments = state.full_n_segments().expect("failed to get number of segments");
        for i in 0..num_segments {
            let segment = state.full_get_segment_text(i).expect("failed to get segment");
            let start_timestamp = state.full_get_segment_t0(i).expect("failed to get start");
            let end_timestamp = state.full_get_segment_t1(i).expect("failed to get end");

            println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
        }

        let et = std::time::Instant::now();
        println!("-> Finished (took {}ms)", (et - st).as_millis());
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