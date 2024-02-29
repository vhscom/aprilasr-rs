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
        // Assuming a straightforward conversion is possible
        SpeakerID { data: t.data }
    }
}

/// Implements the `Into` trait for converting `SpeakerID` into the low-level FFI representation `afi::AprilSpeakerID`.
///
/// This `Into` implementation allows seamless conversion of a Rust-friendly `SpeakerID` into the corresponding
/// low-level FFI representation used by `afi::AprilSpeakerID`.
impl Into<afi::AprilSpeakerID> for SpeakerID {
    fn into(self) -> afi::AprilSpeakerID {
        afi::AprilSpeakerID { data: self.data }
    }
}

#[cfg(test)]
mod tests {
    use md5::compute;

    use super::*;

    #[test]
    fn provides_speaker_interface() {
        // This is a UTF-16LE encoding. The first two bytes 0xAB and 0xCD represent
        // the byte order mark for little-endian UTF-16 (BOM). The remaining bytes
        // are the encoded text as UTF-16LE characters, which are 2-byte code units
        // that consist of a high surrogate followed by a low surrogate.
        let encoded = [
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xBA, 0xDC, 0xFE, 0x1A, 0x2B, 0x3C,
            0x4D, 0x5E,
        ];

        #[allow(unused_mut)]
        let mut max_speakers = SpeakerID { data: encoded };
        assert_eq!(max_speakers.data.len(), 16);

        // Do needless things to demonstrate ways to do useful things.
        let cyphertext_speakers = vec![
            "3e6e450acf34e9f3333bfdadb516e533", // echo Jane | md5sum
            "0f36f95c7f1ddfc81ea827400c4a7c2c", // echo John | md5sum
            "2fc1c0beb992cd7096975cfebf9d5c3b", // echo Bob | md5sum
        ];
        let search_value = cyphertext_speakers[1];
        match cyphertext_speakers
            .iter()
            .position(|name| name == &"61409aa1fd47d4a5332de23cbf59a36f")
        {
            Some(index) => println!("Found {} at index {}", search_value, index),
            None => println!("{} not found in list", search_value),
        };

        // Do needless things to demonstrate ways to do useful things.
        let speakers = vec!["Jane", "John", "Bob"];
        let mut speaker_ids = Vec::new();
        for speaker in speakers {
            println!("Encrypting {}", speaker);
            let hash = compute(speaker);
            let id = SpeakerID { data: hash.0 };
            speaker_ids.push(id);
            println!("Added new speaker with hash {:?}", hash);
        }
    }
}
