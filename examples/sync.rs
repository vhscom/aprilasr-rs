//!
//! aprilasr - rust bindings for the april-asr C api (libaprilasr)
//! Copyright (C) 2024  VHS <vhsdev@tutanota.com>
//!
//! This file is part of aprilasr.
//!
//! aprilasr is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! aprilasr is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!

//! # April ASR Library Example
//!
//! This example Rust file showcases basic usage of the April ASR library.
//!
//! ## Setup
//!
//! 1. Run `./makewav.sh` to create a sample English wavefile.
//! 1. Run `./getmodel.sh` to download the English April model.
//! 1. Then run `cargo run --example sync` to run this file.
//!
//! ## Usage
//!
//! Run this file to see the basic functionality of the April ASR library in action.

// Import the April ASR library
use aprilasr::{init_april_api, Model, ResultType, Session, Token};

use std::fs::File;
use std::io::{self, Read};
use std::sync::Once;

/// Hard-coded path to wav file. Generate wav file by script
/// using the makewave.sh shell script in the project source.
const WAV_FILE_PATH: &str = "mono_16bit16khz.wav";

/// April model you wish to use. Download a model using the
/// getmodel.sh shell script in the project source or using
/// any of the download links in the April ASR documentation.
const APRIL_MODEL_PATH: &str = "april-english-dev-01110_en.april";

/// Initialize the April API with version 1 one time only.
///
/// The function uses the `call_once` method on a static `INIT` variable. Within the closure
/// passed to `call_once`, it invokes the `init_april_api` function, passing the provided version
/// as an argument. This initialization pattern is common for scenarios where certain operations
/// need to be performed only once, such as initializing global resources.
fn initialize() {
    static INIT: Once = Once::new();
    INIT.call_once(|| init_april_api(1));
}

/// Example callback for ASR results, prints recognition information to the console.
///
/// # Parameters
///
/// * `result_type` - ASR result type received from the engine.
fn example_handler(result_type: ResultType) {
    let (prefix, tokens_str) = match result_type {
        ResultType::RecognitionFinal(tokens) => ("@ ", tokens_to_string(tokens.unwrap())),
        ResultType::RecognitionPartial(tokens) => ("- ", tokens_to_string(tokens.unwrap())),
        ResultType::CantKeepUp | ResultType::Silence | ResultType::Unknown => (".", String::new()),
    };
    println!("{}{}", prefix, tokens_str);
}

/// Converts a vector of `Token` instances into a single string.
///
/// # Parameters
///
/// * `tokens` - Vector of `Token` instances.
///
/// # Returns
///
/// A `String` containing the concatenated textual representations of the tokens.
fn tokens_to_string(tokens: Vec<Token>) -> String {
    tokens.iter().map(|t| t.token()).collect()
}

/// Main function demonstrating basic usage of the April ASR library.
fn main() -> Result<(), io::Error> {
    initialize(); // Initialize April ASR. Required to load a Model.

    // Read the entire contents of the WAV file into a buffer
    let mut buffer = Vec::new();
    File::open(WAV_FILE_PATH)?.read_to_end(&mut buffer)?;

    // Add 1 second of padding (16,000 samples for 16kHz audio)
    let padding_samples = 40800; // 2.55s
    buffer.extend(vec![0; padding_samples * 2]); // PCM16 has 2 bytes per sample

    // Load an April ASR model from a file
    let model = Model::new(APRIL_MODEL_PATH).unwrap();

    // Print model metadata
    println!("Model name: {}", model.name());
    println!("Model desc: {}", model.description());
    println!("Model lang: {}", model.language());
    println!("Model samplerate: {}", model.sample_rate());

    println!();

    if let Ok(session) = Session::new(&model, example_handler, false, true) {
        // Feed PCM16 audio data to the session
        session.feed_pcm16(&buffer);
    }

    println!();
    println!();
    println!("done");

    Ok(())
}
