use base64::{Engine as _, engine::general_purpose};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound;
use std::sync::{Arc, Mutex};

pub fn record_audio() -> String {
    let host = cpal::default_host();
    let device = match host.default_input_device() {
        Some(d) => d,
        None => return "".to_string(),
    };
    let config = match device.default_input_config() {
        Ok(c) => c,
        Err(_) => return "".to_string(),
    };
    let channels = config.channels();
    let sample_rate = config.sample_rate().0;
    let samples_clone = Arc::new(Mutex::new(Vec::new()));
    let samples_clone2 = Arc::clone(&samples_clone);
    let stream = match device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &_| {
            let mut s = samples_clone2.lock().unwrap();
            for &sample in data {
                let sample_i16 = (sample * i16::MAX as f32) as i16;
                s.push(sample_i16);
            }
        },
        |err| eprintln!("Audio error: {}", err),
        None,
    ) {
        Ok(s) => s,
        Err(_) => return "".to_string(),
    };
    let _ = stream.play();
    std::thread::sleep(std::time::Duration::from_secs(10));
    let _ = stream.pause();
    drop(stream);
    let samples = Arc::try_unwrap(samples_clone).unwrap().into_inner().unwrap();
    let spec = hound::WavSpec {
        channels: channels as u16,
        sample_rate: sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut buf = Vec::new();
    {
        let cursor = std::io::Cursor::new(&mut buf);
        let mut writer = hound::WavWriter::new(cursor, spec).unwrap();
        for sample in samples {
            writer.write_sample(sample).unwrap();
        }
        writer.finalize().unwrap();
    }
    general_purpose::STANDARD.encode(buf)
}