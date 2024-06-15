// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Converts the RON string to the JavaScript value.

use ron::Value;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

/// Parses a RON string, constructing the JavaScript value or object described
/// by the string.
///
/// # Errors
///
/// Returns an error if any of the following are true:
///
/// - The string to parse is not valid RON.
/// - Cannot construct the JavaScript value or object from a RON value.
#[wasm_bindgen]
pub fn parse(text: &str) -> Result<JsValue, JsError> {
    let value: Value = ron::from_str(text)?;
    value
        .serialize(&Serializer::json_compatible())
        .map_err(JsError::from)
}
