use std::{env, path::PathBuf, process::Command, str};

use anyhow::{anyhow, bail};
use regex::Regex;

fn main() -> anyhow::Result<()> {
    if env::var("CARGO_FEATURE_STATIC").is_ok() {
        build()?;
    } else if cfg!(target_os = "macos") {
        link_homebrew().or_else(|_| link_pkg_config())?;
    } else if cfg!(target_os = "windows") {
        unimplemented!()
    } else {
        link_pkg_config()?;
    }
    Ok(())
}

fn build() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("upstream");
    if !src_dir.join("Makefile").exists() {
        bail!("OpenBLAS submodule isn't checked out. Run git submodule update -i?");
    }
    let num_jobs = env::var("NUM_JOBS");
    let mut build = Command::new("make");
    build.arg("libopenlibm.a").current_dir(&src_dir);
    if let Ok(j) = num_jobs {
        build.arg(format!("-j{}", j));
    }
    build.status()?;
    Command::new("make")
        .arg(format!("prefix={}", out_dir.display()))
        .arg("install")
        .current_dir(&src_dir)
        .status()?;

    println!("cargo:include={}", out_dir.join("include").display());
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=openlibm");
    Ok(())
}

fn link_pkg_config() -> anyhow::Result<()> {
    let options = Command::new("pkg-config")
        .args(&["--cflags-only-I", "--libs-only-L", "openlibm"])
        .output()?
        .stdout;
    let options = str::from_utf8(&options)?;
    let re = Regex::new(r"-I([^ ]+)").unwrap();
    let include_path = re
        .captures(options)
        .ok_or_else(|| anyhow!("-I option is missing"))?
        .get(1)
        .unwrap()
        .as_str();
    let re = Regex::new(r"-L([^ ]+)").unwrap();
    let lib_search_path = re
        .captures(options)
        .ok_or_else(|| anyhow!("-L option is missing"))?
        .get(1)
        .unwrap()
        .as_str();
    println!("cargo:include={}", include_path);
    println!("cargo:rustc-link-search=native={}", &lib_search_path);
    println!("cargo:rustc-link-lib=openlibm");
    Ok(())
}

fn link_homebrew() -> anyhow::Result<()> {
    let prefix = Command::new("brew")
        .args(&["--prefix", "openlibm"])
        .output()?
        .stdout;
    let prefix = dbg!(str::from_utf8(&prefix)?.trim_end());
    println!("cargo:include={}/include", &prefix);
    println!("cargo:rustc-link-search=native={}/lib", &prefix);
    println!("cargo:rustc-link-lib=openlibm");
    Ok(())
}
