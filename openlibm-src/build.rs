use std::{env, path::PathBuf, process::Command};

fn main() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("upstream");
    Command::new("make")
        .arg("libopenlibm.a")
        .current_dir(&src_dir)
        .status()?;
    Command::new("make")
        .arg(format!("prefix={}", out_dir.display()))
        .arg("install-static")
        .current_dir(&src_dir)
        .status()?;

    println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=openlibm");
    Ok(())
}
