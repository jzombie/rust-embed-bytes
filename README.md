# build-resource-byte-arrays (Work in Progress)

`build-resource-byte-arrays` is a Rust crate that simplifies working with generated byte array resources during the build process of other crates.

It is intended to help avoid the complexity of relaying `OUT_DIR` from `build.rs` to the compilation step of consuming crates.

-- WORK IN PROGRESS --

Related: https://stackoverflow.com/questions/75676449/how-to-build-and-publish-a-crate-containing-generated-code

TODO: Document this and the workaround

```bash
error: couldn't read `src/../target/debug/build/ticker-sniffer-82f0d34d30344691/out/bin/COMPRESSED_COMPANY_SYMBOL_LIST_BYTE_ARRAY.bin`: No such file or directory (os error 2)
 --> src/dynamic_resources.rs:7:63
  |
7 | pub static COMPRESSED_COMPANY_SYMBOL_LIST_BYTE_ARRAY: &[u8] = include_bytes!("../target/debug/build/ticker-sniffer-82f0d34d30344691/out/bin/COMPRESSED_COMPANY_SYMBOL_LIST_BYTE_ARRAY.bin");
  |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in the macro `include_bytes` (in Nightly builds, run with -Z macro-backtrace for more info)
```
