# Sled WASI Test

This is a test project to iron out bugs with Sled in a `wasm32-wasi` environment.

Currently there are issues with system time, PIDs, and disk access.  Other unknown issues probably exist as well.

## Build/Test Commands

```
./scripts/build
./scripts/test
./scripts/test_log   // logs all WASI API calls
```

## Tested Against

https://github.com/TorchlightSoftware/sled/tree/wasm32-wasi

Cargo.toml has sled dependency pointing to a local symlink `sled`.  Remove this symlink and replace it with the appropriate target you wish to test against.

## Resolved

* std::process::id()
* std::env::temp_dir()
* std::env::current_dir()

## Current Issues

* file.try_clone()
