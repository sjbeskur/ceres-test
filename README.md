# Ceres Solver test
This is just an example for building and using the dependency as I wanted to see if it cross compiles for arm


### To build x86

```
cargo build 
```

### To build aarch64
Install prereqs
```
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
rustup target add aarch64-unknown-linux-gnu

```

Build
```
cargo build --release --target aarch64-unknown-linux-gnu
```

Strip (optional)
```
aarch64-linux-gnu-strip target/aarch64-unknown-linux-gnu/release/ceres-test
```
