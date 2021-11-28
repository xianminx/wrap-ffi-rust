# Rust FFI: Wrapping C API in Rust struct

See also
* [Rust FFI: Wrapping C API in Rust struct](http://vojtech.kral.hk/en/rust-ffi-wrapping-c-api-in-rust-struct/)

* [Rust FFI](https://doc.rust-lang.org/stable/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code)

Sample c source code is at `src/clang/foo.c`, and is built by rust [cc crate](https://docs.rs/cc/1.0.72/cc/), see `build.rs`.

```rs
    cc::Build::new()
        .file("src/clang/foo.c")
        .compile("foo");
```

This build script will compile `src/clang/foo.c` and place the output `libfoo.a` under `target/debug/build/wrap-ffi-rust-78a3d465fdbd7435/out/`, then tells cargo to link against `static=foo`. Of course, you can specify link type as dynamic by adding flags to `build.rs`.
```rs

        .shared_flag(true)
        .static_flag(true)
```

```sh
target/debug/build/wrap-ffi-rust-78a3d465fdbd7435/output
15:running: "cc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m64" "-arch" "x86_64" "-Wall" "-Wextra" "-o" "/Users/lucas/dev/workspace/rust/wrap-ffi-rust/target/debug/build/wrap-ffi-rust-78a3d465fdbd7435/out/src/clang/foo.o" "-c" "src/clang/foo.c"
21:running: "ar" "cq" "/Users/lucas/dev/workspace/rust/wrap-ffi-rust/target/debug/build/wrap-ffi-rust-78a3d465fdbd7435/out/libfoo.a" "/Users/lucas/dev/workspace/rust/wrap-ffi-rust/target/debug/build/wrap-ffi-rust-78a3d465fdbd7435/out/src/clang/foo.o"
23:running: "ar" "s" "/Users/lucas/dev/workspace/rust/wrap-ffi-rust/target/debug/build/wrap-ffi-rust-78a3d465fdbd7435/out/libfoo.a"
25:cargo:rustc-link-lib=static=foo
```


## Run
```sh
➜  wrap-ffi-rust git:(master) ✗ cargo run       
warning: unused manifest key: toolchain
   Compiling wrap-ffi-rust v0.1.0 (/Users/lucas/dev/workspace/rust/wrap-ffi-rust)
warning: clang: warning: argument unused during compilation: '-shared' [-Wunused-command-line-argument]
warning: type `foo` should have an upper camel case name
 --> src/ffi.rs:5:10
  |
5 | pub type foo = *mut c_void;
  |          ^^^ help: convert the identifier to upper camel case (notice the capitalization): `Foo`
  |
  = note: `#[warn(non_camel_case_types)]` on by default

warning: use of deprecated function `std::mem::uninitialized`: use `mem::MaybeUninit` instead
  --> src/foo.rs:26:36
   |
26 |       let mut foo: ffi::foo = mem::uninitialized();
   |                                    ^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

warning: `wrap-ffi-rust` (bin "wrap-ffi-rust") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 1.73s
     Running `target/debug/wrap-ffi-rust`
foo_create in c
foo_bar in c
main done
foo_destroy in c
```