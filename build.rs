fn main() {
    cc::Build::new()
        .file("src/clang/foo.c")
        .shared_flag(true)
        .compile("foo");
}
