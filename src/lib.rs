mod audio;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn run_audio_processing() {
    let shared_buffer = Arc::new(Mutex::new(Vec::new()));

    let sample_rate = audio::initialize_audio(shared_buffer.clone());

    let input_thread = thread::spawn(move || {
        audio::simulate_audio_input(sample_rate, shared_buffer);
    });

    loop {
        audio::process_output_buffer(&shared_buffer);
    }

    input_thread.join().expect("Input thread panicked");
}

