use std::env;

fn main() {
    cc::Build::new()
        .file("c/fast-eint.c")
        .static_flag(true)
        .flag("-O3")
        .flag("-Wall")
        .flag("-Werror")
        .compile("fast-eint-c-impl");
}
