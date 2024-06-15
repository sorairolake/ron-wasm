// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, missing_docs)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn integer() {
    {
        let value = ron_wasm::parse("42").map_err(JsValue::from).unwrap();
        assert_eq!(value, JsValue::from(42));
    }

    {
        let value = ron_wasm::parse("-2_147_483_648")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(i32::MIN));
    }
    {
        let value = ron_wasm::parse("2_147_483_647")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(i32::MAX));
    }

    {
        let value = ron_wasm::parse("-9007199254740991")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(-9_007_199_254_740_991.0));
    }
    {
        let value = ron_wasm::parse("9007199254740991")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(9_007_199_254_740_991.0));
    }
}

#[wasm_bindgen_test]
fn binary() {
    {
        let value = ron_wasm::parse("0b1111111111111111111111111111111")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(i32::MAX));
    }
    {
        let value = ron_wasm::parse("0b111_1111_1111_1111_1111_1111_1111_1111")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(i32::MAX));
    }
}

#[wasm_bindgen_test]
fn octal() {
    {
        let value = ron_wasm::parse("0o17777777777")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(i32::MAX));
    }
    {
        let value = ron_wasm::parse("0o17_777_777_777")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(i32::MAX));
    }
}

#[wasm_bindgen_test]
fn hex() {
    {
        let value = ron_wasm::parse("0x7fffffff")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(i32::MAX));
    }
    {
        let value = ron_wasm::parse("0x7fff_ffff")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(i32::MAX));
    }

    {
        let value = ron_wasm::parse("0x7FFFFFFF")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(i32::MAX));
    }
    {
        let value = ron_wasm::parse("0x7FFF_FFFF")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(i32::MAX));
    }
}

#[wasm_bindgen_test]
fn float() {
    {
        let value = ron_wasm::parse("3.141592653589793")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(std::f64::consts::PI));
    }

    {
        let value = ron_wasm::parse("NaN").map_err(JsValue::from).unwrap();
        assert!(value.as_f64().map(f64::is_nan).unwrap_or_default());
    }
    {
        let value = ron_wasm::parse("inf").map_err(JsValue::from).unwrap();
        assert_eq!(value, JsValue::from(f64::INFINITY));
    }
    {
        let value = ron_wasm::parse("-inf").map_err(JsValue::from).unwrap();
        assert_eq!(value, JsValue::from(f64::NEG_INFINITY));
    }
}

#[wasm_bindgen_test]
fn exp() {
    {
        let value = ron_wasm::parse("-1.7976931348623157e308")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(f64::MIN));
    }

    {
        let value = ron_wasm::parse("-1.7976931348623157E308")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(f64::MIN));
    }

    {
        let value = ron_wasm::parse("1.7976931348623157e308")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(f64::MAX));
    }

    {
        let value = ron_wasm::parse("1.7976931348623157E308")
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, JsValue::from(f64::MAX));
    }
}

#[wasm_bindgen_test]
fn string() {
    let value = ron_wasm::parse(r#""RON""#).map_err(JsValue::from).unwrap();
    assert_eq!(value, JsValue::from("RON"));
}

#[wasm_bindgen_test]
fn character() {
    let value = ron_wasm::parse(r"'a'").map_err(JsValue::from).unwrap();
    assert_eq!(value, JsValue::from("a"));
}

#[wasm_bindgen_test]
fn boolean() {
    {
        let value = ron_wasm::parse("true").map_err(JsValue::from).unwrap();
        assert_eq!(value, JsValue::TRUE);
    }
    {
        let value = ron_wasm::parse("false").map_err(JsValue::from).unwrap();
        assert_eq!(value, JsValue::FALSE);
    }
}

#[wasm_bindgen_test]
fn optional() {
    {
        let value = ron_wasm::parse("Some(42)").map_err(JsValue::from).unwrap();
        assert_eq!(value, JsValue::from(42));
    }
    {
        let value = ron_wasm::parse("None").map_err(JsValue::from).unwrap();
        assert_eq!(value, JsValue::UNDEFINED);
    }
}
