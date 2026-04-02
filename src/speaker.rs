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

/// Unique identifier for a speaker.
///
/// This struct represents a unique identifier for a speaker, which can be used as a
/// hash of the speaker's name or other distinguishing characteristics. The identifier
/// can be provided to [`aas_create_session`](fn.aas_create_session.html) for saving and
/// restoring state associated with the speaker. If the identifier is set to all zeros,
/// it will be ignored.
///
/// Please note that as of now, the functionality related to `SpeakerID` is not
/// fully implemented, and setting or using the speaker ID may have no effect.
///
/// # Example
///
/// ```rust
/// use aprilasr::SpeakerID;
///
/// // Create a new SpeakerID
/// let speaker_id = SpeakerID {
///     data: [0; 16], // All zeros (ignored in the current implementation)
/// };
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(missing_docs)]
pub struct SpeakerID {
    pub data: [u8; 16usize],
}

/// Implements the `Default` trait for `SpeakerID`.
///
/// The `Default` implementation creates a `SpeakerID` instance with all zeros in the `data` field.
impl Default for SpeakerID {
    fn default() -> Self {
        Self { data: [0; 16] }
    }
}

/// Provides a conversion from low-level FFI bindings [`afi::AprilSpeakerID`] to [`SpeakerID`].
///
/// # Safety
///
/// The function assumes that the provided [`afi::AprilSpeakerID`] is valid and properly initialized.
/// Incorrect or uninitialized values may lead to undefined behavior.
impl From<afi::AprilSpeakerID> for SpeakerID {
    fn from(t: afi::AprilSpeakerID) -> Self {
        SpeakerID { data: t.data }
    }
}

/// Provides a conversion from [`SpeakerID`] to the low-level FFI representation [`afi::AprilSpeakerID`].
impl From<SpeakerID> for afi::AprilSpeakerID {
    fn from(val: SpeakerID) -> Self {
        afi::AprilSpeakerID { data: val.data }
    }
}

#[cfg(test)]
mod tests {
    use md5::compute;

    use super::*;

    #[test]
    fn provides_speaker_interface() {
        let encoded = [
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xBA, 0xDC, 0xFE, 0x1A, 0x2B, 0x3C,
            0x4D, 0x5E,
        ];

        let speaker = SpeakerID { data: encoded };
        assert_eq!(speaker.data.len(), 16);

        let default_speaker = SpeakerID::default();
        assert_eq!(default_speaker.data, [0; 16]);

        let hash = compute("Jane");
        let hashed_speaker = SpeakerID { data: hash.0 };
        assert_ne!(hashed_speaker.data, [0; 16]);
    }
}
