use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn initialize_audio(shared_buffer: Arc<Mutex<Vec<f32>>>) -> f32 {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available");
    let config = device.default_output_config().expect("Failed to get default output config");

    let sample_rate = config.sample_rate().0 as f32;

    let channels = config.channels() as usize;

    let shared_buffer_clone = shared_buffer.clone();

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &_| {
                let buffer = &mut *shared_buffer_clone.lock().unwrap();
                for sample in data.chunks(channels) {
                    let input = sample[0]; // Assuming mono audio input
                    buffer.push(input);
                }
            },
            move |err| eprintln!("Error occurred on stream: {}", err),
        )
        .expect("Failed to build output stream");

    stream.play().expect("Failed to play stream");

    sample_rate
}

pub fn simulate_audio_input(sample_rate: f32, shared_buffer: Arc<Mutex<Vec<f32>>>) {
    let mut time = 0.0;
    loop {
        let input = (time.sin() * 0.5) as f32;
        time += 440.0 / sample_rate;
        thread::sleep(Duration::from_secs_f32(1.0 / sample_rate));
        let mut buffer = shared_buffer.lock().unwrap();
        buffer.push(input);
    }
}

pub fn process_output_buffer(shared_buffer: &Arc<Mutex<Vec<f32>>>) {
    let mut buffer = shared_buffer.lock().unwrap();
    if !buffer.is_empty() {
        println!("{:?}", buffer);
        buffer.clear();
    }
}

