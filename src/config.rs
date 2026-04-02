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

use crate::SpeakerID;

/// Enumeration of April configuration flags.
///
/// This enum represents various configuration flags that can be used with the April library.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(i32)]
pub enum ConfigFlagBits {
    /// Represents the zero bit.
    Zero,

    /// If set, the input audio should be fed in real-time (1 second of audio per second) in small chunks.
    /// Calls to `aas_feed_pcm16` and `aas_flush` will be fast as it will delegate processing to a background thread.
    /// The handler will be called from the background thread at some point later.
    /// The accuracy may be degraded depending on the system hardware.
    /// You may get an accuracy estimate by calling `aas_realtime_get_speedup`.
    AsyncNoRealtime,

    /// Similar to `AsyncNoRealtime`, but does not degrade accuracy depending on system hardware.
    /// However, if the system is not fast enough to process audio, the background thread will fall behind,
    /// results may become unusable, and the handler will be called with `APRIL_RESULT_ERROR_CANT_KEEP_UP`.
    AsyncRealtime,
}

/// Provides a conversion from low-level FFI bindings [`afi::AprilConfigFlagBits`] to [`ConfigFlagBits`].
///
/// # Safety
///
/// The function assumes that the provided [`afi::AprilConfigFlagBits`] is a valid variant of the enum.
/// Incorrect or invalid values may lead to undefined behavior.
impl From<afi::AprilConfigFlagBits> for ConfigFlagBits {
    fn from(bit: afi::AprilConfigFlagBits) -> Self {
        match bit {
            afi::AprilConfigFlagBits_APRIL_CONFIG_FLAG_ASYNC_NO_RT_BIT => {
                ConfigFlagBits::AsyncNoRealtime
            }
            afi::AprilConfigFlagBits_APRIL_CONFIG_FLAG_ASYNC_RT_BIT => {
                ConfigFlagBits::AsyncRealtime
            }
            afi::AprilConfigFlagBits_APRIL_CONFIG_FLAG_ZERO_BIT => ConfigFlagBits::Zero,
            _ => unreachable!("Unexpected AprilConfigFlagBits"),
        }
    }
}

/// Provides a conversion from [`ConfigFlagBits`] to the low-level FFI representation [`afi::AprilConfigFlagBits`].
impl From<ConfigFlagBits> for afi::AprilConfigFlagBits {
    fn from(val: ConfigFlagBits) -> Self {
        val as afi::AprilConfigFlagBits
    }
}

/// Configuration for the April ASR system.
///
/// This struct encapsulates the configuration parameters for the April ASR system,
/// providing a flexible setup for customization. It includes the speaker identifier,
/// recognition result handler, user data, and configuration flags.
///
/// # Fields
///
/// - `speaker`: Unique identifier for the speaker. This can be utilized as a hash
///   of the speaker's name or other distinguishing characteristics. It is used in
///   conjunction with [`aas_create_session`](fn.aas_create_session.html) for saving
///   and restoring state associated with the speaker.
///
/// - `handler`: The handler that will be called as recognition events occur. This
///   may be invoked from a different thread, so appropriate synchronization mechanisms
///   should be employed if necessary.
///
/// - `userdata`: A pointer to user-specific data that can be associated with the
///   configuration. This data is passed along to the recognition result handler,
///   allowing users to pass additional information as needed.
///
/// - `flags`: Configuration flags represented by [`ConfigFlagBits`]. These flags
///   provide options for adjusting the behavior of the ASR system, such as enabling
///   real-time processing or specifying how asynchronous processing should be handled.
///
/// # Safety
///
/// Creating a `Config` instance assumes that the provided values in the `afi::AprilConfig` are valid
/// and properly initialized. Incorrect or uninitialized values may lead to undefined behavior.
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(PartialEq, Debug)]
pub struct Config {
    speaker: SpeakerID,

    /// The handler that will be called as events occur. This may be called from a different thread.
    handler: afi::AprilRecognitionResultHandler,
    userdata: *mut ::std::os::raw::c_void,

    /// See [`ConfigFlagBits`].
    flags: ConfigFlagBits,
}

impl Config {
    /// Creates a new configuration with the provided parameters.
    pub fn new(
        speaker: SpeakerID,
        handler: afi::AprilRecognitionResultHandler,
        userdata: *mut ::std::os::raw::c_void,
        flags: ConfigFlagBits,
    ) -> Config {
        Config {
            speaker,
            handler,
            userdata,
            flags,
        }
    }

    /// Gets the speaker identifier.
    ///
    /// # Returns
    ///
    /// A reference to the speaker identifier.
    pub fn speaker(&self) -> &SpeakerID {
        &self.speaker
    }

    /// Gets the recognition result handler.
    ///
    /// # Returns
    ///
    /// The recognition result handler.
    pub fn handler(&self) -> afi::AprilRecognitionResultHandler {
        self.handler
    }

    /// Gets the user data.
    ///
    /// # Returns
    ///
    /// A pointer to the user data.
    pub fn userdata(&self) -> *mut ::std::os::raw::c_void {
        self.userdata
    }

    /// Gets the configuration flags.
    ///
    /// # Returns
    ///
    /// The configuration flags.
    pub fn flags(&self) -> ConfigFlagBits {
        self.flags
    }
}

/// Conversion from low-level FFI representation (`afi::AprilConfig`) to the Rust-friendly `Config`.
///
/// This implementation enables the creation of a `Config` instance based on the low-level FFI representation
/// provided by the `afi::AprilConfig` type.
///
/// # Safety
///
/// This function assumes that the incoming `afi::AprilConfig` value is valid and properly initialized.
/// Using incorrect or uninitialized values may result in undefined behavior.
impl From<afi::AprilConfig> for Config {
    /// Converts a `afi::AprilConfig` into a `Config` instance.
    ///
    /// # Arguments
    ///
    /// * `cfg` - The low-level FFI representation of `Config` to be converted.
    ///
    /// # Panics
    ///
    /// Panics if the creation of `Config` fails. This typically occurs when the provided
    /// `afi::AprilConfig` values result in an invalid configuration.
    fn from(cfg: afi::AprilConfig) -> Self {
        Config::new(
            SpeakerID::from(cfg.speaker),
            cfg.handler,
            cfg.userdata,
            ConfigFlagBits::from(cfg.flags),
        )
    }
}

/// Provides a conversion from [`Config`] to the low-level FFI representation [`afi::AprilConfig`].
impl From<Config> for afi::AprilConfig {
    fn from(config: Config) -> Self {
        afi::AprilConfig {
            speaker: config.speaker.into(),
            handler: config.handler,
            userdata: config.userdata,
            flags: config.flags.into(),
        }
    }
}

/// Builder for creating and customizing `Config` instances with default or specific parameters.
///
/// This builder pattern is designed to offer ergonomic and efficient configuration creation
/// by using mutable references.
pub struct ConfigBuilder {
    speaker: SpeakerID,
    handler: afi::AprilRecognitionResultHandler,
    userdata: *mut ::std::os::raw::c_void,
    flags: ConfigFlagBits,
}

impl ConfigBuilder {
    /// Creates a new `ConfigBuilder` with default values.
    pub fn new() -> Self {
        Self {
            speaker: SpeakerID::default(),
            handler: afi::AprilRecognitionResultHandler::default(),
            userdata: ::std::ptr::null_mut(),
            flags: ConfigFlagBits::AsyncNoRealtime,
        }
    }

    /// Sets the speaker ID.
    pub fn speaker(&mut self, speaker: SpeakerID) -> &mut Self {
        self.speaker = speaker;
        self
    }

    /// Sets the recognition result handler.
    pub fn handler(&mut self, handler: afi::AprilRecognitionResultHandler) -> &mut Self {
        self.handler = handler;
        self
    }

    /// Sets the user-specific data.
    pub fn userdata(&mut self, userdata: *mut ::std::os::raw::c_void) -> &mut Self {
        self.userdata = userdata;
        self
    }

    /// Sets the configuration flags.
    pub fn flags(&mut self, flags: ConfigFlagBits) -> &mut Self {
        self.flags = flags;
        self
    }

    /// Builds the `Config` instance.
    pub fn build(&self) -> Config {
        Config::new(self.speaker, self.handler, self.userdata, self.flags)
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{handler_cb_wrapper, ResultType};

    use super::*;

    #[test]
    fn test_config_builder() {
        // Test default configuration
        let default_config = ConfigBuilder::new().build();
        let expected_default = Config {
            speaker: SpeakerID::default(),
            handler: None,
            userdata: ::std::ptr::null_mut(),
            flags: ConfigFlagBits::AsyncNoRealtime,
        };
        assert_eq!(default_config, expected_default);

        // Define a mock handler_callback for testing
        #[allow(unused_variables)]
        fn handler_callback(result_type: ResultType) {
            unimplemented!()
        }

        // Test custom configuration with handler callback
        let custom_config = ConfigBuilder::new()
            .speaker(SpeakerID { data: [42; 16] })
            .handler(Some(handler_cb_wrapper))
            .userdata(handler_callback as *mut std::os::raw::c_void)
            .flags(ConfigFlagBits::Zero)
            .build();
        let expected_custom = Config {
            speaker: SpeakerID { data: [42; 16] },
            handler: Some(handler_cb_wrapper),
            userdata: handler_callback as *mut std::os::raw::c_void,
            flags: ConfigFlagBits::Zero,
        };
        assert_eq!(custom_config, expected_custom);

        // Test builder can pass self by mutable reference
        let mut config_builder = ConfigBuilder::new();
        config_builder.speaker(SpeakerID::default());
        config_builder.flags(ConfigFlagBits::AsyncRealtime);
        let mut_config = config_builder.build();
        let expected_mut = Config {
            speaker: SpeakerID::default(),
            handler: None,
            userdata: ::std::ptr::null_mut(),
            flags: ConfigFlagBits::AsyncRealtime,
        };
        assert_eq!(mut_config, expected_mut);
    }
}
