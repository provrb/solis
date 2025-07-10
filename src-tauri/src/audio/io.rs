use cpal::traits::{DeviceTrait, HostTrait};

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
