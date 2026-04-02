# aprilasr

High-level Rust wrapper for the [april-asr] C API (libaprilasr), providing safe, ergonomic bindings for offline speech-to-text. Built on [aprilasr-sys].

## Features

- Safe wrappers around the april-asr C library with automatic resource cleanup
- Synchronous and asynchronous recognition modes
- Multiple concurrent sessions sharing a single model
- Zero-copy audio feeding on little-endian platforms
- Panic-safe FFI callback handling

## Requirements

- [aprilasr-sys] (pulled automatically via Cargo)
- libonnxruntime
- libclang
- An [April ASR model file](https://april.sapples.net/)

## Quick start

```rust
use aprilasr::{init_april_api, Model, ResultType, Session};

fn main() {
    init_april_api(1);

    let model = Model::new("april-english-dev-01110_en.april").unwrap();

    let callback = |result: ResultType| match result {
        ResultType::RecognitionFinal(tokens) => {
            let text: String = tokens.iter().map(|t| t.token()).collect();
            println!("{}", text);
        }
        _ => {}
    };

    let session = Session::new(&model, callback, true, true).unwrap();
    session.feed_pcm16(&[]); // feed real PCM16 audio here
}
```

## Examples

Download a model and generate a test audio file, then run any of the included examples:

```sh
./getmodel.sh
./makewav.sh
cargo run --example sync
cargo run --example async
cargo run --example multi
```

## Testing

```sh
./getmodel.sh  # required: downloads the English model
cargo test
```

## License

[GPL-3.0-or-later](COPYING)

[april-asr]: https://github.com/abb128/april-asr
[aprilasr-sys]: https://crates.io/crates/aprilasr-sys
