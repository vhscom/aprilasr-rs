# aprilasr

High-level wrapper for the [april-asr] C api (libaprilasr) using [aprilasr-sys].

Read the documentation here: https://abb128.github.io/april-asr/

```rust
use aprilasr::{init_april_api, Model, Session};

fn main() {
    init_april_api(1);

    let model = Model::new("april-english-dev-01110_en.april").unwrap();

    let asynchronous = true;
    let no_rt = true;
    let callback = |result_type| println!("{:?}", result_type);

    let session = Session::new(
        &model, callback, asynchronous, no_rt
    ).unwrap();

    session.feed_pcm16(&[]);
}
```

Run tests and example after building [aprilasr-sys]:

```sh
$ ./getmodel.sh
$ cargo test
$ ./makewav.sh
$ cargo run --example sync
```

[april-asr]: https://github.com/abb128/april-asr
[aprilasr-sys]: https://crates.io/crates/aprilasr-sys
