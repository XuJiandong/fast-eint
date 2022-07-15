fn main() {
    cc::Build::new()
        .file("c/fast-eint.c")
        .static_flag(true)
        .flag("-O3")
        .flag("-Wall")
        .flag("-Werror")
        //        .flag_if_supported("-march=haswell")
        .compile("fast-eint-c-impl");
}
