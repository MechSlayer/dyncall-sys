use std::path::PathBuf;

use git2::Repository;

const DYNCALL_REPOSITRY: &'static str = "https://github.com/MechSlayer/dyncall.git";

#[cfg(feature = "dyncall")]
const INCLUDE_DYNCALL: bool = true;
#[cfg(not(feature = "dyncall"))]
const INCLUDE_DYNCALL: bool = false;

#[cfg(feature = "dyncallback")]
const INCLUDE_DYNCALLBACK: bool = true;
#[cfg(not(feature = "dyncallback"))]
const INCLUDE_DYNCALLBACK: bool = false;

#[cfg(feature = "dynload")]
const INCLUDE_DYNLOAD: bool = true;
#[cfg(not(feature = "dynload"))]
const INCLUDE_DYNLOAD: bool = false;

#[cfg(feature = "std")]
const USE_STD: bool = true;
#[cfg(not(feature = "std"))]
const USE_STD: bool = false;

fn main() {
    let out_dir = std::env::var("OUT_DIR").map(PathBuf::from).unwrap();
    let repo_dir = out_dir.join("dyncall_repo");
    if repo_dir.exists() {
        std::fs::remove_dir_all(&repo_dir).expect("failed to remove dyncall");
    }

    Repository::clone(DYNCALL_REPOSITRY, &repo_dir).expect("failed to clone dyncall");

    let dst = cmake::build(&repo_dir);
    let lib_dir = dst.join("lib");
    let include_dir = dst.join("include");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    let dyncall_header = include_dir.join("dyncall.h");
    let dyncallback_header = include_dir.join("dyncall_callback.h");
    let dynload_header = include_dir.join("dynload.h");

    if INCLUDE_DYNCALL {
        println!("cargo:rustc-link-lib=dyncall");
    }

    if INCLUDE_DYNCALLBACK {
        println!("cargo:rustc-link-lib=dyncallback");
    }
    if INCLUDE_DYNLOAD {
        println!("cargo:rustc-link-lib=dynload");
    }

    let bindings = bindgen::builder();

    let bindings = if INCLUDE_DYNCALL {
        bindings.header(dyncall_header.to_str().unwrap())
    } else {
        bindings
    };
    let bindings = if INCLUDE_DYNCALLBACK {
        bindings.header(dyncallback_header.to_str().unwrap())
    } else {
        bindings
    };
    let bindings = if INCLUDE_DYNLOAD {
        bindings.header(dynload_header.to_str().unwrap())
    } else {
        bindings
    };
    let bindings = if !USE_STD {
        bindings.use_core()
    } else {
        bindings
    };
    let bindings = bindings.generate().expect("unable to generate bindings");

    let gen_path = out_dir.join("bindings_dyncall.rs");
    bindings
        .write_to_file(gen_path)
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=build.rs");
}
