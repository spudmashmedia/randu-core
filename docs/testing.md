# Testing Instructions

## Testing Rust Logic with Cargo

The following will test the private methods located in [lib.rs](../src/lib.rs):

```
randu-core> cargo test
```

## Testing WASM Module with wasm-pack

The following commands will perform Integration tests against the public functions exposed from the WASM module.

A headless browser will be used to perform the test located [tests/web.rs](../tests/web.rs)

```
randu-core> wasm-pack test --headless --chrome
randu-core> wasm-pack test --headless --firefox
```

*please note --safari (non headless) is an option, but you must **enable Develop | Allow Remote Automation**

## Run Demo web integration Node.js / React.js / Next.js
```
randu-core> cd www
randu-core> npm i
randu-core> npm run start
```

Open browser at http://localhost:3000
