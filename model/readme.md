# Model documentation
* https://www.nasdaq.com/docs/SoupBinTCP%204.1.pdf

# Local build
```shell
cargo nextest run --all-features
cargo nextest run --examples
cargo test --doc
cargo doc
cargo clippy --all-features -- --deny warnings
```