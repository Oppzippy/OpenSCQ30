### Configuring Cargo
rustflags are set by build scripts, but rust-analyzer won't see that. In order to get the wasm build working with rust-analyzer, create `.cargo/config.toml` in the root of this repository.
```toml
[build]
rustflags = ["--cfg=web_sys_unstable_apis"]
```

For faster non wasm/android builds, also consider using the mold linker and split debuginfo:
```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = [
    "--cfg=web_sys_unstable_apis",
    "-C",
    "link-arg=-fuse-ld=/usr/bin/mold",
    "-C",
    "split-debuginfo=unpacked",
]

```
