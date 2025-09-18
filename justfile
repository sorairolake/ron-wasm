# SPDX-FileCopyrightText: 2024 Shun Sakai
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

alias lint := clippy

# Run default recipe
_default:
    just -l

# Build a package
build:
    cargo build

# Remove generated artifacts
clean:
    cargo clean

# Check a package
check:
    wasm-pack build -s sorairolake -t nodejs --dev

# Run tests
test:
    wasm-pack test --node

# Run the formatter
fmt:
    cargo +nightly fmt

# Run the linter
clippy:
    cargo clippy -- -D warnings

# Apply lint suggestions
clippy-fix:
    cargo clippy --fix --allow-dirty --allow-staged -- -D warnings

# Build examples for the Wasm bindings
build-wasm-examples:
    wasm-pack build -t deno

# Run `deno fmt`
fmt-wasm-examples:
    deno fmt deno.jsonc examples/*.ts

# Run `deno lint`
lint-wasm-examples:
    deno lint examples/*.ts

# Run `deno check`
type-check-wasm-examples:
    deno check examples/*.ts

# Run the linter for GitHub Actions workflow files
lint-github-actions:
    actionlint -verbose

# Run the formatter for the README
fmt-readme:
    npx prettier -w README.md

# Build the Wasm bindings
build-wasm:
    wasm-pack build -s sorairolake -t nodejs --release

# Publish the Wasm bindings
publish-wasm: build-wasm
    wasm-pack publish -a public

# Increment the version
bump part:
    bump-my-version bump {{ part }}
    cargo set-version --bump {{ part }}
