// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Converts the JavaScript value to the RON string.

use ron::Value;
use wasm_bindgen::{JsError, JsValue, prelude::wasm_bindgen};

/// Converts a JavaScript value to a RON string.
///
/// # Errors
///
/// Returns an error if any of the following are true:
///
/// - Cannot convert `value` to a RON value.
/// - Cannot convert a RON value to a RON string.
#[wasm_bindgen]
pub fn stringify(value: JsValue) -> Result<String, JsError> {
    let value: Value = serde_wasm_bindgen::from_value(value)?;
    ron::to_string(&value).map_err(JsError::from)
}
