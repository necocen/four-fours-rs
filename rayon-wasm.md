# Wasm with rayon

CAVEAT: Currently this is very slow.

To use rayon with wasm, you can use [wasm-bindgen-rayon](https://github.com/GoogleChromeLabs/wasm-bindgen-rayon) crate.
This crate has several limitations so you need to follow some steps to use.

1. Use nightly-2021-07-29 toolchain. You can use `rustup override set nightly-2021-07-29` on repository root directory, or specify version with rust-toolchain.toml file.
2. Set some feature flags. You can set them with `.cargo/config.toml` file like below.

```toml
[target.wasm32-unknown-unknown]
rustflags = ["-C", "target-feature=+atomics,+bulk-memory,+mutable-globals"]

[unstable]
build-std = ["panic_abort", "std"]
```

3. Pass `--target web` option to `wasm-pack` command.
4. Open `js/wasm.ts` and uncomment these lines:

```ts
    // await fourFours.default(); // init
    // await fourFours.initThreadPool(navigator.hardwareConcurrency);
```

5. Build and run it.

```sh
wasm-pack build --target web --features with-rayon
cd js
npm i
npm start
```
