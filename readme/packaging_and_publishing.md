# bench
```shell
cargo bench
```

# local build
```shell
cargo nextest run --all-features &&
cargo nextest run --examples --all-features &&
cargo test --doc --all-features &&
RUSTDOCFLAGS="-D warnings" cargo  doc --all-features &&
cargo clippy --all-features -- --deny warnings
```
