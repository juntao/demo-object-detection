# Demonstrate object detection with mediapipe-rs and TensorflowLite

Install Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-wasi
```

Install WasmEdge + WASI-NN + TFLite

```
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | sudo bash -s -- --plugins wasi_nn-tensorflowlite -p /usr/local
```

Build

```
cargo build --target wasm32-wasi --release
wasmedge compile target/wasm32-wasi/release/demo-object-detection.wasm demo-object-detection.wasm
```

Run

```
wasmedge --dir .:. demo-object-detection.wasm example.jpg output.jpg
```

The annotated image is in `output.jpg` and the console output is:

```
DetectionResult:
  Detection #0:
    Box: (left: 0.47665566, top: 0.05484602, right: 0.87270254, bottom: 0.87143743)
    Category #0:
      Category name: "dog"
      Display name:  None
      Score:         0.7421875
      Index:         18
  Detection #1:
    Box: (left: 0.12402746, top: 0.37931007, right: 0.5297544, bottom: 0.8517805)
    Category #0:
      Category name: "cat"
      Display name:  None
      Score:         0.7421875
      Index:         17
```
