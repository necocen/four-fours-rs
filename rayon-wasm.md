# Wasm with rayon

CAVEAT: Currently this is very slow and does not work correctly.

To use rayon with wasm, you can use [wasm-bindgen-rayon](https://github.com/RReverser/wasm-bindgen-rayon) crate.
This crate has several limitations so you need to follow some steps to use.

1. Use nightly toolchain. You can use `rustup run nightly` to use specific toolchain when you run `wasm-pack``.
2. Set some feature flags. You can set them with `.cargo/config.toml` file like below.

```toml
[target.wasm32-unknown-unknown]
rustflags = ["-C", "target-feature=+atomics,+bulk-memory,+mutable-globals"]

[unstable]
build-std = ["panic_abort", "std"]
```

3. Open `js/src/wasm.ts` and uncomment these lines:

```ts
    // await fourFours.default(); // init
    // await fourFours.initThreadPool(navigator.hardwareConcurrency);
```

4. Pass `--target web` and `--features with-rayon` options to `wasm-pack` command.
5. Build and run it.

```sh
rustup run nightly wasm-pack build --target web --features with-rayon
cd js
npm i
npm start
```
