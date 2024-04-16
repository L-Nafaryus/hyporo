use std::{env, fs, path::Path};

use git2::{build::RepoBuilder, FetchOptions};

fn main() {
    let occt_version: &str = "V7_8_1"; // &env::var("OCCT_VERSION").expect("Set env variable OCCT_VERSION");

    let current_dir = env::current_dir().expect("Should have 'current' directory");
    let patch_dir = current_dir.join("patch");

    let out = env::var("OUT_DIR").expect("Should have 'out' directory");
    let out_dir = Path::new(&out);
    let target_dir = out_dir.join("opencascade");
    let lib_dir = out_dir.join("lib");
    let include_dir = out_dir.join("include");

    let url = "https://github.com/Open-Cascade-SAS/OCCT.git";

    let mut fetch_options = FetchOptions::new();
    fetch_options.depth(1);
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options).branch("master");

    println!("cargo:rustc-env=OCCT_VERSION={}", occt_version);

    if !target_dir.is_dir() {
        //fs::remove_dir_all(target_dir.clone()).expect("Failed to remove directory");

        let repo = builder.clone(url, &target_dir).expect("");

        let refname = "V7_8_1";
        let (object, reference) = repo
            .revparse_ext(refname)
            .unwrap_or_else(|_| panic!("Should have ref: {}", refname));

        repo.checkout_tree(&object, None)
            .expect("Failed to checkout");

        match reference {
            Some(gref) => repo.set_head(gref.name().unwrap()),
            None => repo.set_head_detached(object.id()),
        }
        .expect("Failed to set HEAD");
    }

    //if !(lib_dir.exists() && include_dir.exists()) {
    cmake::Config::new(target_dir)
        .define("BUILD_PATCH", patch_dir)
        .define("BUILD_LIBRARY_TYPE", "Static")
        .define("USE_D3D", "OFF")
        .define("USE_DRACO", "OFF")
        .define("USE_EIGEN", "OFF")
        .define("USE_FFMPEG", "OFF")
        .define("USE_FREEIMAGE", "OFF")
        .define("USE_FREETYPE", "OFF")
        .define("USE_GLES2", "OFF")
        .define("USE_OPENGL", "OFF")
        .define("USE_OPENVR", "OFF")
        .define("USE_RAPIDJSON", "OFF")
        .define("USE_TBB", "OFF")
        .define("USE_TCL", "OFF")
        .define("USE_TK", "OFF")
        .define("USE_VTK", "OFF")
        .define("USE_XLIB", "OFF")
        .define("INSTALL_DIR_LIB", lib_dir.clone())
        .define("INSTALL_DIR_INCLUDE", "include") //include_dir.clone())
        .build();
    //}

    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_str().unwrap()
    );

    for entry in fs::read_dir(lib_dir).unwrap() {
        if entry.is_err() {
            continue;
        }
        let path = entry.unwrap().path();
        if path.is_file() {
            let name = path.file_stem().unwrap().to_str().unwrap();
            println!("cargo:rustc-link-lib=static={}", &name[3..]);
        }
    }

    println!("cargo:rustc-link-lib=static=wrapper");
    println!("cargo:warning={}", include_dir.display());

    cxx_build::bridge("src/lib.rs")
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .define("_USE_MATH_DEFINES", "TRUE")
        .include(include_dir)
        .include("src")
        .compile("wrapper");

    println!("cargo:rerun-if-env-changed=OCCT_VERSION");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
}
