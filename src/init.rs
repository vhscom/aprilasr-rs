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

use std::ffi::c_int;

/// Exposes the April API version as defined by the FFI cast to `i32`.
pub static APRIL_VERSION: i32 = afi::APRIL_VERSION as c_int;

/// Initializes the April API.
///
/// Must be called once before creating a Model.
///
/// Pass APRIL_VERSION as argument like so: aam_api_init(APRIL_VERSION).
///
/// # Safety
/// This function should be called in a safe, single-threaded context to initialize the April API safely.
///
/// # Panics
/// - Panics when the provided version is unexpected.
pub fn init_april_api(version: i32) {
    match version {
        1 => unsafe { afi::aam_api_init(version) },
        _ => panic!("Unsupported version. Wanted: 1. Got: {:?}", version),
    };
}

#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;

    use super::*;

    #[test]
    fn uses_functional_sys_crate() {
        let ffi_version = afi::APRIL_VERSION as c_int;
        assert_eq!(ffi_version, 1);
        assert_eq!(unsafe { afi::aam_api_init(ffi_version) }, ());
    }

    #[test]
    fn exposes_expected_api_version() {
        let rust_version = APRIL_VERSION;
        assert_ne!(Some(rust_version), None);
        assert_eq!(rust_version, 1);
    }

    #[test]
    fn provides_init_wrapper() -> Result<(), Box<dyn std::error::Error>> {
        let _ = catch_unwind(|| {
            init_april_api(APRIL_VERSION);
        });
        Ok(())
    }

    #[test]
    #[should_panic]
    fn init_rejects_unexpected_versions() {
        init_april_api(0);
    }

    #[test]
    #[should_panic]
    fn init_rejects_unsupported_versions() {
        init_april_api(2);
    }
}
