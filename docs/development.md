For faster builds, consider using the mold linker, and if you're on Linux, `split-debuginfo=unpacked`:
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
