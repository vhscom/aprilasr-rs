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

use std::ffi::{CStr, CString};

/// Wrapper for managing an April ASR model running in memory.
///
/// This struct provides a safe Rust interface for interacting with the April ASR model.
/// It is responsible for managing the lifecycle of the underlying model, including creation,
/// retrieval of information (name, description, language, sample rate), and destruction.
///
/// # Safety
///
/// The `Model` struct implements the [`Drop`] trait to ensure proper resource cleanup.
///
/// The implementation of the `Drop` trait guarantees that resources associated with the
/// April ASR model are released correctly when a `Model` instance goes out of scope.
///
/// Users should ensure that all instances of `Model` are properly managed and that no
/// references to the model are held beyond their intended lifespan to prevent resource leaks.
///
/// # Examples
///
/// Example usage of the `Model` struct can be found in the module's documentation.
///
/// [`Drop`]: std::ops::Drop
#[derive(Debug)]
pub struct Model {
    pub(crate) ctx: *mut afi::AprilASRModel_i,
}

impl Model {
    /// Instantiate an April ASR model given a file path.
    ///
    /// # Arguments
    ///
    /// * `model_path` - The file path to the April ASR model.
    ///
    /// # Errors
    ///
    /// Returns an error if the model cannot be created from the provided file path.
    pub fn new(model_path: &str) -> Result<Model, Box<dyn std::error::Error>> {
        let path = CString::new(model_path).expect("CString::new failed");
        let model = unsafe { afi::aam_create_model(path.as_ptr()) };

        if model.is_null() {
            Err("Failed to create ASR model".into())
        } else {
            Ok(Model { ctx: model })
        }
    }

    /// Get the name of the model.
    ///
    /// # Safety
    /// Guarantees a freshly-owned `String` allocation.
    pub fn name(&self) -> String {
        let cstr = unsafe { CStr::from_ptr(afi::aam_get_name(self.ctx)) };
        String::from_utf8_lossy(cstr.to_bytes()).to_string()
    }

    /// Get the description of the model.
    ///
    /// # Safety
    ///
    /// Guarantees a freshly-owned `String` allocation.
    pub fn description(&self) -> String {
        let cstr = unsafe { CStr::from_ptr(afi::aam_get_description(self.ctx)) };
        String::from_utf8_lossy(cstr.to_bytes()).to_string()
    }

    /// Get the language of the model.
    ///
    /// # Safety
    ///
    /// Guarantees a freshly-owned `String` allocation.
    pub fn language(&self) -> String {
        let cstr = unsafe { CStr::from_ptr(afi::aam_get_language(self.ctx)) };
        String::from_utf8_lossy(cstr.to_bytes()).to_string()
    }

    /// Get the sample rate of the model.
    pub fn sample_rate(&self) -> usize {
        unsafe { afi::aam_get_sample_rate(self.ctx) }
    }
}

/// Implementation of the `Drop` trait for the `Model` struct.
///
/// The `Drop` trait defines a method named `drop` that is called when the value
/// goes out of scope. In this implementation, it is used to release the resources
/// associated with the April ASR model, ensuring proper cleanup.
///
/// # Safety
///
/// The `afi::aam_free` function is marked as `unsafe` because it deals with raw
/// pointers and memory management. The implementation assumes that the
/// `aprilasr_sys` crate provides a safe and correct way to free the resources
/// associated with the ASR model. Incorrect usage of this function or invalid
/// pointers may result in undefined behavior.
impl Drop for Model {
    /// Drops the April ASR model, releasing associated resources.
    fn drop(&mut self) {
        unsafe { afi::aam_free(self.ctx) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{init_april_api, APRIL_VERSION};
    use std::ffi::{CStr, CString};
    use std::ptr::null_mut;

    use super::*;

    #[test]
    fn can_load_model() {
        init_april_api(APRIL_VERSION);

        let path_str = CString::new("april-english-dev-01110_en.april").expect("CString::new failed");
        let model = unsafe { afi::aam_create_model(path_str.as_ptr()) };
        assert_ne!(model, null_mut());

        let description = unsafe {
            CStr::from_ptr(afi::aam_get_description(model))
                .to_string_lossy()
                .to_string()
        };
        assert_eq!(description, "Punctuation + Numbers 23a3");

        let name = unsafe {
            CStr::from_ptr(afi::aam_get_name(model))
                .to_string_lossy()
                .to_string()
        };
        assert_eq!(name, "April English Dev-01110");

        let language = unsafe {
            CStr::from_ptr(afi::aam_get_language(model))
                .to_string_lossy()
                .to_string()
        };
        assert_eq!(language, "en");

        assert_eq!(unsafe { afi::aam_get_sample_rate(model) }, 16000);

        unsafe { afi::aam_free(model) }
    }

    #[test]
    fn cannot_load_fake_model() {
        init_april_api(APRIL_VERSION);

        let path_str = CString::new("invalid.april").expect("CString::new failed");
        let model = unsafe { afi::aam_create_model(path_str.as_ptr()) };
        assert_eq!(model, null_mut());
    }

    #[test]
    fn wraps_model() {
        init_april_api(APRIL_VERSION);

        let model_path = "april-english-dev-01110_en.april";
        let model = Model::new(model_path).unwrap();

        assert_eq!(model.name(), "April English Dev-01110");
        assert_eq!(model.description(), "Punctuation + Numbers 23a3");
        assert_eq!(model.language(), "en");
        assert_eq!(model.sample_rate(), 16000);

        // Drop trait automatically frees model memory.
    }
}
