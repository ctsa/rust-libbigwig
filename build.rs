fn main() {
    let src = [
        "src/libBigWig/bwRead.c",
        "src/libBigWig/bwValues.c",
        "src/libBigWig/bwWrite.c",
        "src/libBigWig/io.c",
    ];
    let mut cfg = cc::Build::new();
    cfg.files(src.iter())
       .include("src/libBigWig/include")
       .flag("-O3")
       .flag("-Wno-pointer-arith")
       .flag("-Wno-unused-parameter")
       .define("NOCURL", None);
    if let Ok(lib) = pkg_config::Config::new().atleast_version("1.2").probe("zlib") {
        for path in &lib.include_paths {
            cfg.include(path);
        }
    }
    cfg.compile("lib_micro_bigwig");

    for &s in src.iter() {
        println!("cargo:rerun-if-changed={}", s);
    }
}
