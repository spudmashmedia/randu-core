# randu-core

A Rust lang built Web Assembly (WASM) module that integrates with randomuser.me API.

Compatible with Deno / JavaScript / Node / TypeScript.

This module demonstrates:
- Rust lang to Web Assmbely using **wasm-pack** to:
  - generate a Strongly Typed proxy library for randomuser.me
  - integrate with wasm-bindgen (using web-sys for web browser Fetch API)
- Rust lang Unit Testing
- wasm-pack Integration testing against the generated wasm code
- GitHub Actions & Packages for CI/CD integration and NPM package storage


# Build Status
|Branch| Build Status|
|---|---|
|master| ![Rust](https://github.com/spudmashmedia/randu-core/workflows/Rust/badge.svg)|

# Getting Started
- [Prequisite Before Starting](/docs/prerequisite.md)
- [Build Instructions](/docs/build.md)
- [Testing Instructions](/docs/testing.md)
- [CICD Pipeline](/docs/cicd.md)
  
# Documentation
  
- [The Purpose of this Repository](/docs/purpose.md)
- [References](/docs/references.md)

# License
This code is distributed under the terms and conditions of the [MIT License](/LICENSE)
