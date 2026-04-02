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

/// This module provides a Rust interface for interacting with the April ASR library,
/// allowing developers to leverage speech-to-text capabilities in Rust applications.
use aprilasr_sys::ffi as afi;

use crate::{handler_cb_wrapper, ConfigBuilder, ConfigFlagBits, Model, ResultType, SpeakerID};

/// Wrapper for managing an April ASR session running in memory.
///
/// The `Session` struct encapsulates the functionality of an ASR session and provides methods for interacting with it.
/// It is responsible for managing the session's lifecycle, including creation and automatic resource cleanup upon dropping.
///
/// # Safety
///
/// The `Session` struct is marked as `unsafe` because it encapsulates low-level operations, and misuse can lead to
/// undefined behavior. It implements the [`Drop`] trait to ensure proper resource cleanup when a `Session` instance goes out of scope.
///
/// The `Session` holds a raw pointer `ctx` to the underlying April ASR session, and it is the responsibility of the user
/// to ensure that the associated resources are properly managed and that the session is used safely within the constraints
/// of the April ASR library.
///
/// The `Session` also holds a reference to the `Model` using a simple reference (`&'a Model`). This ensures that the `Model`
/// is not deallocated before the associated `Session` instances are closed. The ownership and lifecycle management of the `Model`
/// are abstracted away, providing a safe way to share the model among multiple sessions.
///
/// Users should ensure that all instances of `Session` are properly managed and that no
/// references to the session are held beyond their intended lifespan to prevent resource leaks.
///
/// # Examples
///
/// Example usage of the `Session` struct can be found in the module's documentation.
///
/// [`Drop`]: std::ops::Drop
#[derive(Debug)]
pub struct Session<'a> {
    pub(crate) ctx: *mut afi::AprilASRSession_i,
    // Hold onto the model reference
    _model: &'a Model,
}

impl<'a> Session<'a> {
    /// Initializes a new ASR session with the specified ASR model and configuration.
    ///
    /// # Safety
    ///
    /// The safety of this function relies on the correctness of the underlying FFI library's
    /// `aas_create_session` function. Incorrect usage or invalid parameters may result in
    /// undefined behavior.
    ///
    /// The `Session` holds a borrowed reference to the provided [`Model`], ensuring
    /// the model is not dropped before the associated `Session` instances are closed.
    ///
    /// # Arguments
    ///
    /// * `model` - The [`Model`] to be used for the session.
    /// * `callback` - A callback function to handle the result of the ASR session asynchronously.
    /// * `asynchronous` - A flag indicating whether the ASR session should run asynchronously.
    /// * `no_rt` - A flag indicating whether real-time processing should be disabled.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the newly created `Session` instance or an error message.
    pub fn new(
        model: &'a Model,
        callback: fn(result: ResultType) -> (),
        asynchronous: bool,
        no_rt: bool,
    ) -> Result<Session<'a>, Box<dyn std::error::Error>> {
        let mut config_builder = ConfigBuilder::new();

        config_builder.flags(match (asynchronous, no_rt) {
            (true, true) => ConfigFlagBits::AsyncNoRealtime,
            (true, false) => ConfigFlagBits::AsyncRealtime,
            _ => ConfigFlagBits::Zero,
        });
        config_builder.userdata(callback as *mut std::os::raw::c_void);
        config_builder.handler(Some(handler_cb_wrapper));
        config_builder.speaker(SpeakerID::default()); // No speaker by default

        let config = config_builder.build();
        let session = unsafe { afi::aas_create_session(model.ctx, config.into()) };

        if session.is_null() {
            Err("Failed to create ASR session".into())
        } else {
            Ok(Session {
                ctx: session,
                _model: model,
            })
        }
    }

    /// Processes any unprocessed samples and produces a final result.
    ///
    /// # Safety
    ///
    /// The safety of this function depends on the correctness of the `aas_flush` function
    /// from the underlying FFI (Foreign Function Interface) library. Incorrect usage or
    /// invalid parameters may lead to undefined behavior.
    pub fn flush(&self) {
        unsafe { afi::aas_flush(self.ctx) };
    }

    /// Feed PCM16 audio samples to the session.
    ///
    /// This method takes a vector of 8-bit PCM audio samples (`pcm16_samples`) and converts them
    /// to 16-bit signed integers (`i16`). It ensures that every consecutive pair of bytes is
    /// mapped to a single i16 sample using little-endian byte order.
    ///
    /// # Arguments
    ///
    /// * `pcm16_samples` - A vector of 8-bit PCM audio samples.
    ///
    /// # Safety
    ///
    /// The method internally uses unsafe code to feed the PCM16 audio data to the session.
    /// Ensure that the provided `pcm16_bytes` vector is valid and adheres to the specified format.
    ///
    /// The PCM16 audio data must be single-channel and sampled according to the sample rate
    /// obtained from `aam_get_sample_rate`.
    ///
    /// Note: `short_count` in the `aas_feed_pcm16` call represents the number of shorts, not bytes.
    pub fn feed_pcm16(&self, pcm16_bytes: &[u8]) {
        // On little-endian targets, try zero-copy reinterpret if aligned.
        #[cfg(target_endian = "little")]
        {
            // SAFETY: align_to checks that the pointer is properly aligned for i16.
            // The C library only reads from the pointer (never writes through it).
            let (prefix, shorts, suffix) = unsafe { pcm16_bytes.align_to::<i16>() };
            if prefix.is_empty() && suffix.len() < 2 {
                unsafe {
                    afi::aas_feed_pcm16(self.ctx, shorts.as_ptr() as *mut i16, shorts.len());
                }
                return;
            }
        }

        // Fallback: convert bytes to i16 samples (handles big-endian or unaligned input).
        let mut audio_samples: Vec<i16> = pcm16_bytes
            .chunks_exact(2)
            .map(|bytes| i16::from_le_bytes([bytes[0], bytes[1]]))
            .collect();
        unsafe { afi::aas_feed_pcm16(self.ctx, audio_samples.as_mut_ptr(), audio_samples.len()) };
    }

    /// Gets the speedup factor for realtime processing.
    ///
    /// If the `ConfigFlagBits::AsyncRealtime` flag is set, this method returns a floating-point
    /// number describing how much audio is being sped up to keep up with realtime processing.
    /// If the number is below `1.0`, audio is not being sped up. If it is greater than `1.0`,
    /// the audio is being sped up, and the accuracy may be reduced.
    ///
    /// # Safety
    ///
    /// This method is marked as unsafe because it relies on the correctness of the underlying
    /// FFI (Foreign Function Interface) call to `afi::aas_realtime_get_speedup`. Incorrect usage
    /// or invalid parameters may lead to undefined behavior.
    ///
    /// # Returns
    ///
    /// The speedup factor for realtime processing.
    pub fn realtime_get_speedup(&self) -> f32 {
        unsafe { afi::aas_realtime_get_speedup(self.ctx) }
    }
}

/// Implementation of the `Drop` trait for the `Session` struct.
///
/// The `Drop` trait defines a method named `drop` that is called when the value
/// goes out of scope. In this implementation, it is used to free the resources
/// associated with the ASR session, ensuring proper cleanup.
///
/// # Safety
///
/// The `afi::aas_free` function is marked as `unsafe` because it deals with raw
/// pointers and memory management. The implementation assumes that the
/// `aprilasr_sys` crate provides a safe and correct way to free the resources
/// associated with the ASR session. Incorrect usage of this function or invalid
/// pointers may result in undefined behavior.
impl<'a> Drop for Session<'a> {
    fn drop(&mut self) {
        unsafe {
            afi::aas_free(self.ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{init_april_api, Model, APRIL_VERSION};

    use super::*;

    #[test]
    fn wraps_session() {
        init_april_api(APRIL_VERSION);

        let model = Model::new("april-english-dev-01110_en.april").unwrap();

        let session = Session::new(
            &model,
            |result_type| println!("{:?}", result_type),
            true,
            true,
        )
        .unwrap();

        assert!(session.ctx.is_null() == false);

        // Drop traits automatically free session and model memory.
    }

    #[test]
    fn feeds_pcm16_to_session() {
        init_april_api(APRIL_VERSION);

        let model = Model::new("april-english-dev-01110_en.april").unwrap();
        let asynchronous = true;
        let no_rt = true;
        let callback = |result_type| println!("{:?}", result_type);
        let session = Session::new(&model, callback, asynchronous, no_rt).unwrap();

        session.feed_pcm16(&[]);

        // Drop traits automatically free session and model memory.
    }

    #[test]
    fn test_session_can_be_flushed() {
        init_april_api(APRIL_VERSION);

        let model = Model::new("april-english-dev-01110_en.april").unwrap();
        let asynchronous = true;
        let no_rt = true;
        let callback = |result_type| println!("{:?}", result_type);
        let session = Session::new(&model, callback, asynchronous, no_rt).unwrap();

        // Sessions must be fed before being flushed
        session.feed_pcm16(&[]);
        session.flush();

        // Drop traits automatically free session and model memory.
    }

    #[test]
    fn test_session_can_check_speedup() {
        init_april_api(APRIL_VERSION);

        let model = Model::new("april-english-dev-01110_en.april").unwrap();
        let asynchronous = true;
        let no_rt = true;
        let callback = |result_type| println!("{:?}", result_type);
        let session = Session::new(&model, callback, asynchronous, no_rt).unwrap();

        // Expects ConfigFlagBits::AsyncRealtime (asynchronyous=true, no_rt=false)
        let speedup = session.realtime_get_speedup();

        assert_eq!(speedup, 1.0);

        // Drop traits automatically free session and model memory.
    }

    #[test]
    fn test_sessions_can_share_model() {
        init_april_api(APRIL_VERSION);

        let model = Model::new("april-english-dev-01110_en.april").unwrap();
        let asynchronous = true;
        let no_rt = true;
        let callback = |result_type| println!("{:?}", result_type);

        let _ = Session::new(&model, callback, asynchronous, no_rt).unwrap();
        let _ = Session::new(&model, callback, asynchronous, no_rt).unwrap();
    }
}
