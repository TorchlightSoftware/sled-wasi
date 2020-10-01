# Sled WASI Test

This is a test project to iron out bugs with Sled in a `wasm32-wasi` environment.

Currently there are issues with system time, PIDs, and disk access.  Other unknown issues probably exist as well.

Current output:

```
$ cargo +nightly wasi run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `cargo-wasi target/wasm32-wasi/debug/sled-wasi.wasm`
     Running `target/wasm32-wasi/debug/sled-wasi.wasm`
thread 'main' panicked at 'unsupported', library/std/src/sys/wasi/os.rs:183:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Error: failed to run main module `target/wasm32-wasi/debug/sled-wasi.wasm`

Caused by:
    0: failed to invoke command default
    1: wasm trap: unreachable
       wasm backtrace:
         0: 0x266fb4 - <unknown>!__rust_start_panic
         1: 0x217cfc - <unknown>!rust_panic
         2: 0x172cc9 - <unknown>!std::panicking::rust_panic_with_hook::h8ad053d77cc20a20
         3: 0x2603e3 - <unknown>!std::panicking::begin_panic::{{closure}}::hf72e590868c7c6d9
         4: 0x25cfa7 - <unknown>!std::sys_common::backtrace::__rust_end_short_backtrace::hcba35b379f60ffbe
         5: 0x26038d - <unknown>!std::panicking::begin_panic::h891a8b09365982e7
         6: 0x266ad9 - <unknown>!std::sys::wasi::os::getpid::h058eca5a92e03e63
         7: 0x266ef0 - <unknown>!std::process::id::h07f9457c53f8c7ab
         8: 0xe967f - <unknown>!sled::config::Config::gen_temp_path::h07323a6ad7894460
         9: 0x14f290 - <unknown>!<sled::config::Inner as core::default::Default>::default::h0ea6eeca1805843c
         10: 0x21ecd1 - <unknown>!<sled::arc::Arc<T> as core::default::Default>::default::h73d48332e598c6a5
         11: 0x24e4b7 - <unknown>!<sled::config::Config as core::default::Default>::default::h4d8d85430eb27400
         12: 0x266bcd - <unknown>!sled::config::Config::new::h3c72bfc230f9bdf9
         13: 0x1f9c29 - <unknown>!sled::db::open::h7152b2f4549a8ea8
         14: 0xdd3bb - <unknown>!sled_wasi::main::hbc3fa556adc8a45b
         15: 0x25659d - <unknown>!core::ops::function::FnOnce::call_once::h71e5ca860e8e67ef
         16: 0x2550be - <unknown>!std::sys_common::backtrace::__rust_begin_short_backtrace::h03a80d1a10e5bcc3
         17: 0x244585 - <unknown>!std::rt::lang_start::{{closure}}::h2a47a73aa49c2e89
         18: 0x1af9f0 - <unknown>!std::rt::lang_start_internal::hd68a6a98c158ed91
         19: 0x215a7e - <unknown>!std::rt::lang_start::hde61ab384dfacee9
         20: 0x265e2d - <unknown>!__original_main
         21: 0x265e05 - <unknown>!_start

error: failed to execute "wasmtime" "--" "target/wasm32-wasi/debug/sled-wasi.wasm"
    status: exit code: 134
```
