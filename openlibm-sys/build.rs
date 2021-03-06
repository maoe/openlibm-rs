use std::{env, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let include_path = PathBuf::from(env::var("DEP_OPENLIBM_INCLUDE")?).join("openlibm");
    let bindings = bindgen::builder()
        .header(format!("{}", include_path.join("openlibm.h").display()))
        .clang_arg(format!("-I{}", include_path.display()))
        .blacklist_item("^_.*")
        .blacklist_function(".*l$")
        .blacklist_function("nexttoward.?")
        .generate()
        .expect("Unable to generate bindings");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    Ok(())
}
