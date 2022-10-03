use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    // let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    let target_os = get_os_from_triple(target.as_str()).unwrap();
    let target_cpu = get_cpu_from_triple(target.as_str()).unwrap();

    let raylib_source_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("raylib");

    let mut cmake_cfg = cmake::Config::new(raylib_source_path);

    match target_os {
        "darwin" => {
            match target_cpu {
                "aarch64" => {
                    cmake_cfg.define("CMAKE_OSX_ARCHITECTURES", "arm64");
                }
                _ => panic!("build.rs not configured for target cpu: {}", target_cpu),
            };
        }
        _ => {
            panic!("build.rs not configured for target os: {}", target_os);
        }
    }

    let dst = cmake_cfg.build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=raylib");

    match target_os {
        "darwin" => {
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=OpenGL");
        }
        _ => {
            panic!("build.rs not configured for target os: {}", target_os);
        }
    }
}

fn get_os_from_triple(triple: &str) -> Option<&str> {
    triple.splitn(3, '-').nth(2)
}

fn get_cpu_from_triple(triple: &str) -> Option<&str> {
    triple.split('-').next()
}
