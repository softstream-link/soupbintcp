# Content
* [Data Model](model/readme.md) - contains SoupBin data structure bindings
  
# Local build
```shell
cargo nextest run --all-features
cargo nextest run --examples
cargo test --doc
cargo doc
cargo clippy --all-features -- --deny warnings
```