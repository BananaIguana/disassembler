# Disassembler

This is an experimental project to gain an understanding of disassmbly and reassembly.

Only a small subset of aarch64 instructions are currently supported.

Please do not use the contents of this repository in production code or as an instruction encoding/decoding reference as this is purely experimental.

Maybe one day this will change?

If you're curious, make sure you're on the `develop` branch and have pulled from Git LFS.

```
git checkout develop
git lfs pull
```

The project is currently setup as a library so just execute a test:

```
cargo test disassembly_test -- --exact --nocapture
```
