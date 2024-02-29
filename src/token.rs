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

use std::ffi::{c_char, c_float, CStr};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::{fmt, mem, process, slice};

/// Represents flag bits associated with speech recognition result tokens.
///
/// This enum provides information about specific characteristics associated with
/// speech recognition result tokens. It is used to mark the start of a new word
/// or the end of a sentence in the recognized text.
///
/// # Variants
///
/// - `WordBoundary`: If set, this token marks the start of a new word.
///
/// - `SentenceEnd`: If set, this token marks the end of a sentence, meaning the token
///   is equal to ".", "!", or "?". Some models may not have this token.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenFlagBits {
    /// Undocumented feature. This is not found in the flag
    /// bits returned by the Bindgen bindings but it happens.
    Zero,

    /// If set, this token marks the start of a new word.
    // In English, this is equivalent to (token[0] == ' ').
    WordBoundary,

    /// If set, this token marks the end of a sentence, meaning the token is
    /// equal to ".", "!", or "?". Some models may not have this token.
    SentenceEnd,
}

impl From<afi::AprilTokenFlagBits> for TokenFlagBits {
    /// Converts from the FFI representation to the Rust enum.
    ///
    /// # Panics
    ///
    /// Panics if an invalid FFI flag bit value is encountered.
    fn from(flag_bit: afi::AprilTokenFlagBits) -> Self {
        match flag_bit {
            0 => TokenFlagBits::Zero,
            afi::AprilTokenFlagBits_APRIL_TOKEN_FLAG_WORD_BOUNDARY_BIT => {
                TokenFlagBits::WordBoundary
            }
            afi::AprilTokenFlagBits_APRIL_TOKEN_FLAG_SENTENCE_END_BIT => TokenFlagBits::SentenceEnd,
            _ => unreachable!("Unexpected AprilTokenFlagBits"),
        }
    }
}

/// Enumeration of April recognition result types.
///
/// This enum represents different types of recognition results that can be returned by
/// the April library. Each variant provides information about the nature of the recognition
/// result, such as whether it is unknown, partial, final, or indicates an error condition.
///
/// ## Variants
///
/// - `Unknown`: Specifies that the result is unknown.
///
/// - `RecognitionPartial`: Specifies that the result is only partial, and a future call will contain
///   much of the same text but updated. Contains a vector of [`Token`] instances representing
///   the recognized text tokens.
///
/// - `RecognitionFinal`: Specifies that the result is final. Future calls will start from
///   empty and will not contain any of the given text. Contains a vector of [`Token`] instances
///   representing the recognized text tokens.
///
/// - `CantKeepUp`: If in non-synchronous mode, this may be called when the internal audio
///   buffer is full and processing can't keep up. It will be called with count = 0, tokens = `None`.
///
/// - `Silence`: Specifies that there has been some silence. Will not be called repeatedly.
///   It will be called with count = 0, tokens = `None`.
///
/// [`Token`]: enum.Token.html
#[derive(Debug, Clone)]
#[repr(i32)]
pub enum ResultType {
    /// Specifies that the result is unknown.
    Unknown,

    /// Specifies that the result is only partial, and a future call will
    /// contain much of the same text but updated.
    RecognitionPartial(Option<Vec<Token>>),

    /// Specifies that the result is final. Future calls will start from
    /// empty and will not contain any of the given text.
    RecognitionFinal(Option<Vec<Token>>),

    /// If in non-synchronous mode, this may be called when the internal
    /// audio buffer is full and processing can't keep up.
    /// It will be called with count = 0, tokens = `None`.
    CantKeepUp,

    /// Specifies that there has been some silence. Will not be called
    /// repeatedly.
    /// It will be called with count = 0, tokens = `None`.
    Silence,
}

/// Custom error type for errors during Token construction.
///
/// This error type is used to represent errors that may occur during the instantiation
/// of [`Token`](struct.Token.html). It provides additional information about
/// the nature of the error, such as an invalid flag value in the FFI bindings.
///
/// # Example
///
/// ```rust
/// use aprilasr::{Token, TokenError, TokenFlagBits};
///
/// fn create_april_token() -> Result<Token, Box<dyn std::error::Error>> {
///     // Some code that may result in an error during Token creation
///     // ...
///
///     // For simplicity, assume an error condition occurs
///     Err(Box::new(TokenError {
///         message: "Invalid TokenFlagBits value!",
///     }))
/// }
/// ```
#[derive(Debug)]
pub struct TokenError {
    pub message: &'static str,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TokenError {}

/// Represents a speech recognition result token.
///
/// This struct encapsulates information about a speech recognition result token,
/// including the recognized text, log probability, associated flags, and the timestamp
/// at which it was emitted.
#[derive(Debug, Clone)]
pub struct Token {
    token: String,
    logprob: c_float,
    flags: TokenFlagBits,
    time_ms: usize,
}

impl Token {
    /// Instantiates a speech recognition result token.
    ///
    /// # Safety
    ///
    /// This function assumes that the provided `token` pointer is valid and points to a
    /// null-terminated C string. It also assumes that the `flags` parameter is a valid
    /// representation of `afi::AprilTokenFlagBits`. The user should ensure that the input
    /// parameters adhere to these assumptions to prevent undefined behavior.
    ///
    /// The `token` parameter is a C string pointer representing the recognition result token.
    ///
    /// The `logprob` parameter is the log probability of this token being the correct token.
    ///
    /// The `flags` parameter represents the flag bits associated with the token,
    /// and it should be a valid variant of [`TokenFlagBits`](enum.TokenFlagBits.html).
    ///
    /// The `time_ms` parameter denotes the millisecond at which this token was emitted.
    ///
    /// # Returns
    ///
    /// Returns a result containing a newly constructed `Token` instance if the
    /// instantiation is successful. If there are errors during the instantiation,
    /// such as invalid flag values, it returns a boxed error implementing the `Error` trait.
    pub fn new(
        token: *const c_char,
        logprob: c_float,
        flags: afi::AprilTokenFlagBits,
        time_ms: usize,
    ) -> Result<Token, Box<dyn std::error::Error>> {
        let token_cstr = unsafe { CStr::from_ptr(token) };
        let rust_token = String::from_utf8_lossy(token_cstr.to_bytes()).to_string();
        let rust_flags = TokenFlagBits::from(flags);

        Ok(Token {
            token: rust_token,
            logprob,
            flags: rust_flags,
            time_ms,
        })
    }

    /// Returns the recognition result token.
    ///
    /// The returned string contains its own formatting, which may denote the start of
    /// a new word or the next part of a word.
    pub fn token(&self) -> String {
        self.token.clone()
    }

    /// Returns the log probability of this being the correct token.
    pub fn logprob(&self) -> f32 {
        self.logprob
    }

    /// Returns the flag bits associated with the token.
    ///
    /// See [`TokenFlagBits`](enum.TokenFlagBits.html) for possible values.
    pub fn flags(&self) -> TokenFlagBits {
        self.flags
    }

    /// Returns the millisecond at which this token was emitted.
    ///
    /// The counting is based on how much audio is being fed, and time is not advanced
    /// when the session is not given audio.
    pub fn time_ms(&self) -> usize {
        self.time_ms
    }
}

/// Provides a conversion from low-level FFI bindings [`afi::AprilToken`] to [`Token`].
///
/// This implementation allows for more ergonomic conversion directly from low-level FFI bindings.
/// Bear in mind that implementing the `From` trait automatically provides the `Into` trait.
impl From<afi::AprilToken> for Token {
    fn from(t: afi::AprilToken) -> Self {
        Token::new(t.token, t.logprob, t.flags, t.time_ms)
            .unwrap_or_else(|err| panic!("Failed to create Token: {}", err))
    }
}

/// Wrapper function for the C callback `handler_cb_wrapper`.
///
/// This function is intended to be used as a callback from a C API. It translates the C-style
/// callback parameters into Rust types, performs some logic based on the result type, and then
/// invokes a user-provided Rust callback function.
///
/// # Safety
///
/// The callback function is marked as `unsafe` due to the transmutation of the `userdata` pointer
/// and the use of `unsafe` code to work with raw pointers. Users of this function should ensure
/// that the callback function adheres to the expected signature (`fn(ResultType) -> ()`) and
/// that the `userdata` pointer is valid and points to a valid callback function.
///
/// # Arguments
///
/// * `userdata`: A pointer to user-specific data or a callback function.
/// * `result_type`: The result type received from the C API.
/// * `count`: The number of tokens in the `tokens` array.
/// * `tokens`: A pointer to an array of `afi::AprilToken` elements.
///
/// # Panics
///
/// If the closure passed to `catch_unwind` panics, this function prints the panic information
/// to the standard error stream and aborts the process.
pub extern "C" fn handler_cb_wrapper(
    userdata: *mut std::os::raw::c_void,
    result_type: afi::AprilResultType,
    count: usize,
    tokens: *const afi::AprilToken,
) {
    if let Err(e) = catch_unwind(AssertUnwindSafe(|| {
        let result: ResultType = match result_type {
            afi::AprilResultType_APRIL_RESULT_UNKNOWN => ResultType::Unknown,
            afi::AprilResultType_APRIL_RESULT_RECOGNITION_PARTIAL
            | afi::AprilResultType_APRIL_RESULT_RECOGNITION_FINAL => {
                let tokens_slice = unsafe { slice::from_raw_parts(tokens, count) };
                let tokens_vec: Vec<Token> = tokens_slice.iter().map(|t| (*t).into()).collect();
                if result_type == afi::AprilResultType_APRIL_RESULT_RECOGNITION_PARTIAL {
                    ResultType::RecognitionPartial(Some(tokens_vec))
                } else {
                    ResultType::RecognitionFinal(Some(tokens_vec))
                }
            }
            afi::AprilResultType_APRIL_RESULT_ERROR_CANT_KEEP_UP => ResultType::CantKeepUp,
            afi::AprilResultType_APRIL_RESULT_SILENCE => ResultType::Silence,
            _ => unreachable!("Unexpected AprilResultType"),
        };

        let callback: fn(ResultType) -> () = unsafe { mem::transmute(userdata) };
        callback(result);
    })) {
        eprintln!("{:?}", e);
        process::abort();
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn wraps_result_tokens() {
        let result = Token::new(
            b" BATMAN\0".as_ptr(),
            8.73,
            afi::AprilTokenFlagBits_APRIL_TOKEN_FLAG_WORD_BOUNDARY_BIT,
            1705461067638,
        )
        .unwrap();

        assert_eq!(result.token(), " BATMAN");
        assert_eq!(result.logprob(), 8.73);
        assert_eq!(result.flags(), TokenFlagBits::WordBoundary);
        assert_eq!(result.time_ms(), 1705461067638);

        // Do needless things to demonstrate ways to do useful things.
        let mut logprobs = vec![8.73, 4.62, 9.51];
        logprobs.resize_with(5, Default::default);
        assert_eq!(logprobs, [8.73, 4.62, 9.51, 0., 0.]);

        // Do needless things to demonstrate ways to do useful things.
        let mut flag_bits = vec![
            TokenFlagBits::WordBoundary,
            TokenFlagBits::WordBoundary,
            TokenFlagBits::SentenceEnd,
        ];
        flag_bits.resize_with(5, || TokenFlagBits::WordBoundary);
        assert_eq!(
            flag_bits.last().unwrap().clone(),
            TokenFlagBits::WordBoundary
        );

        let another_result = Token {
            token: String::from(" AND"),
            logprob: logprobs[0],
            flags: flag_bits.last().unwrap().clone(),
            time_ms: result.time_ms() + 320,
        };

        assert_eq!(another_result.token(), " AND");
        assert_eq!(another_result.logprob(), 8.73);
        assert_eq!(another_result.flags(), TokenFlagBits::WordBoundary);
        assert_eq!(another_result.time_ms(), 1705461067958);

        let last_result = Token {
            token: String::from(" ROBIN"),
            logprob: logprobs[2],
            flags: TokenFlagBits::SentenceEnd,
            time_ms: another_result.time_ms() + 230,
        };

        assert_eq!(last_result.token(), " ROBIN");
        assert_eq!(last_result.logprob(), 9.51);
        assert_eq!(last_result.flags(), TokenFlagBits::SentenceEnd);
        assert_eq!(last_result.time_ms(), 1705461068188);
    }

    #[test]
    fn creating_april_token_from_valid_input() {
        let valid_token = CString::new("example_token").expect("CString::new failed");
        let logprob = 0.5;
        let valid_flags = afi::AprilTokenFlagBits_APRIL_TOKEN_FLAG_WORD_BOUNDARY_BIT;
        let time_ms = 100;

        match Token::new(valid_token.as_ptr(), logprob, valid_flags, time_ms) {
            Ok(april_token) => {
                assert_eq!(april_token.token, "example_token");
                assert_eq!(april_token.logprob, 0.5);
                assert_eq!(april_token.flags, TokenFlagBits::WordBoundary);
                assert_eq!(april_token.time_ms, 100);
            }
            Err(_) => {
                panic!("Valid token creation failed unexpectedly");
            }
        }
    }

    fn is_recognition_result(result_type: ResultType) -> bool {
        match result_type {
            ResultType::RecognitionFinal(_) | ResultType::RecognitionPartial(_) => true,
            ResultType::Unknown | ResultType::CantKeepUp | ResultType::Silence => false,
        }
    }

    #[test]
    fn test_recognition_result_types() {
        assert!(is_recognition_result(ResultType::RecognitionPartial(None)));
        assert!(is_recognition_result(ResultType::RecognitionFinal(None)));
        assert!(!is_recognition_result(ResultType::Unknown));
        assert!(!is_recognition_result(ResultType::CantKeepUp));
        assert!(!is_recognition_result(ResultType::Silence));
    }
}
