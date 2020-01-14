extern crate cmake;

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let pb_out_dir = out_dir.join("protobuf");
    let onnx_out_dir = out_dir.join("onnx");
    let _ = std::fs::create_dir(&pb_out_dir);
    let _ = std::fs::create_dir(&onnx_out_dir);

    if !pb_out_dir.join("build").join("CMakeCache.txt").exists() {
        cmake::Config::new("third_party/protobuf/cmake")
            .profile("Release")
            .define("protobuf_MSVC_STATIC_RUNTIME", "OFF")
            .define("protobuf_BUILD_TESTS", "OFF")
            .define("protobuf_BUILD_EXAMPLES", "OFF")
            .define("protobuf_BUILD_PROTOC_BINARIES", "ON")
            .define("protobuf_BUILD_SHARED_LIBS", "ON")
            .out_dir(&pb_out_dir)
            .build();
    } else {
        println!("cargo:root={}", pb_out_dir.display());
    }
    println!("cargo:rustc-link-search={}", pb_out_dir.display());
    println!(
        "cargo:rustc-link-search={}",
        pb_out_dir.join("build").display()
    );
    println!(
        "cargo:rustc-link-search={}",
        pb_out_dir.join("build").join("Release").display()
    );
    println!(
        "cargo:rustc-link-search={}",
        pb_out_dir.join("lib").display()
    );

    // Add protoc to `PATH` environment variable
    add_paths(&[
        // Path for Windows
        pb_out_dir.join("bin"),
        // Path for UNIX
        pb_out_dir.join("build"),
    ]);

    // Build ONNX
    if !onnx_out_dir.join("build").join("CMakeCache.txt").exists() {
        cmake::Config::new("third_party/onnx")
            .profile("Release")
            .define("ONNX_ML", "ON")
            .define("ONNX_USE_LITE_PROTO", "ON")
            .cxxflag(format!(
                "-I {}",
                std::fs::canonicalize("third_party/protobuf/src")?.display()
            ))
            .out_dir(&onnx_out_dir)
            .build();
    } else {
        println!("cargo:root={}", onnx_out_dir.display());
    }
    println!(
        "cargo:rustc-link-search={}",
        onnx_out_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=onnx_proto");
    println!("cargo:rustc-link-lib=protobuf-lite");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    Ok(())
}

fn add_paths(extras: &[PathBuf]) {
    std::env::set_var("PATH", create_path(extras));
}

fn create_path(extras: &[PathBuf]) -> String {
    let path = std::env::var_os("PATH").expect("PATH environment variable");
    let paths: Vec<_> = std::env::split_paths(&path).collect();
    std::env::join_paths(&[extras, paths.as_slice()].concat())
        .expect("Join paths")
        .into_string()
        .unwrap()
}
