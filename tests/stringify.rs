// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn integer() {
    {
        let value = ron_wasm::stringify(JsValue::from(42))
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, "42");
    }

    {
        let value = ron_wasm::stringify(JsValue::from(-9_007_199_254_740_991.0))
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, "-9007199254740991");
    }
    {
        let value = ron_wasm::stringify(JsValue::from(9_007_199_254_740_991.0))
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, "9007199254740991");
    }
}

#[wasm_bindgen_test]
fn float() {
    {
        let value = ron_wasm::stringify(JsValue::from(std::f64::consts::PI))
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, "3.141592653589793");
    }

    {
        let value = ron_wasm::stringify(JsValue::from(f64::NAN))
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, "NaN");
    }
    {
        let value = ron_wasm::stringify(JsValue::from(f64::INFINITY))
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, "inf");
    }
    {
        let value = ron_wasm::stringify(JsValue::from(f64::NEG_INFINITY))
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, "-inf");
    }
}

#[wasm_bindgen_test]
fn string() {
    let value = ron_wasm::stringify(JsValue::from("RON"))
        .map_err(JsValue::from)
        .unwrap();
    assert_eq!(value, r#""RON""#);
}

#[wasm_bindgen_test]
fn boolean() {
    {
        let value = ron_wasm::stringify(JsValue::TRUE)
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, "true");
    }
    {
        let value = ron_wasm::stringify(JsValue::FALSE)
            .map_err(JsValue::from)
            .unwrap();
        assert_eq!(value, "false");
    }
}

#[wasm_bindgen_test]
fn null() {
    let value = ron_wasm::stringify(JsValue::NULL)
        .map_err(JsValue::from)
        .unwrap();
    assert_eq!(value, "()");
}

#[wasm_bindgen_test]
fn undefined() {
    let value = ron_wasm::stringify(JsValue::UNDEFINED)
        .map_err(JsValue::from)
        .unwrap();
    assert_eq!(value, "()");
}
