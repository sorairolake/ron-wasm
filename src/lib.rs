// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `ron-wasm` crate is the Wasm bindings for [RON] (Rusty Object Notation).
//!
//! [RON]: https://github.com/ron-rs/ron

#![doc(html_root_url = "https://docs.rs/ron-wasm/0.2.0/")]
// Lint levels of rustc.
#![deny(missing_docs)]

mod parse;
mod stringify;

pub use crate::{parse::parse, stringify::stringify};
