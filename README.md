<!--
SPDX-FileCopyrightText: 2024 Shun Sakai

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# Wasm Bindings for RON

[![CI][ci-badge]][ci-url]
[![npm Version][npm-version-badge]][npm-version-url]
[![crates.io Version][crates-version-badge]][crates-version-url]
![MSRV][msrv-badge]
[![Docs][docs-badge]][docs-url]
![License][license-badge]

**ron-wasm** is the Wasm bindings for [RON] (Rusty Object Notation).

## Usage

### Installation

To install this library:

```sh
npm install @sorairolake/ron-wasm
```

### Build

You will need [`wasm-pack`] to build this crate.

```sh
wasm-pack build
```

This will generate build artifacts in the `pkg` directory.

### Example

```ts
```

### Documentation

See the [documentation][docs-url] for more details.

## Minimum supported Rust version

The minimum supported Rust version (MSRV) of this library is v1.64.0.

## Changelog

Please see [CHANGELOG.adoc].

## Contributing

Please see [CONTRIBUTING.adoc].

## License

Copyright &copy; 2024 Shun Sakai (see [AUTHORS.adoc])

This library is distributed under the terms of either the _Apache License 2.0_
or the _MIT License_.

This project is compliant with version 3.0 of the [_REUSE Specification_]. See
copyright notices of individual files for more details on copyright and
licensing information.

[ci-badge]: https://img.shields.io/github/actions/workflow/status/sorairolake/ron-wasm/CI.yaml?branch=develop&style=for-the-badge&logo=github&label=CI
[ci-url]: https://github.com/sorairolake/ron-wasm/actions?query=branch%3Adevelop+workflow%3ACI++
[npm-version-badge]: https://img.shields.io/npm/v/%40sorairolake%2Fron-wasm?style=for-the-badge&logo=npm
[npm-version-url]: https://www.npmjs.com/package/@sorairolake/ron-wasm
[crates-version-badge]: https://img.shields.io/crates/v/ron-wasm?style=for-the-badge&logo=rust
[crates-version-url]: https://crates.io/crates/ron-wasm
[msrv-badge]: https://img.shields.io/crates/msrv/ron-wasm?style=for-the-badge&logo=rust
[docs-badge]: https://img.shields.io/docsrs/ron-wasm?style=for-the-badge&logo=docsdotrs&label=Docs.rs
[docs-url]: https://docs.rs/ron-wasm
[license-badge]: https://img.shields.io/crates/l/ron-wasm?style=for-the-badge
[RON]: https://github.com/ron-rs/ron
[`wasm-pack`]: https://rustwasm.github.io/wasm-pack/
[CHANGELOG.adoc]: CHANGELOG.adoc
[CONTRIBUTING.adoc]: CONTRIBUTING.adoc
[AUTHORS.adoc]: AUTHORS.adoc
[_REUSE Specification_]: https://reuse.software/spec/
