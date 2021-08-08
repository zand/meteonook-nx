extern crate cbindgen;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn convert(path: PathBuf, out_dir: &Path) {
    let src = fs::read_to_string(&path)
        .unwrap()
        .replace("use web_sys", "//use web_sys")
        .replace("web_sys", "//web_sys")
        .replace("use wasm_bindgen::", "//use wasm_bindgen::")
        .replace("#[wasm_bindgen", "//#[wasm_bindgen")
        .replace("std::", "core::")
        .replace(
            "#[derive(PartialEq, Clone, Copy, Debug)]\npub enum ",
            "#[derive(PartialEq, Clone, Copy, Debug)]\n#[repr(u8)]\npub enum ",
        )
        .replace("\npub fn ", "\n#[no_mangle]\npub extern \"C\" fn ")
        .replace("\npub struct ", "\n#[repr(C)]\npub struct ")
        .replace("\nmacro_rules! ", "\n#[allow(unused_macros)]\nmacro_rules! ");

    fs::write(out_dir.join(path.file_name().unwrap()), src).unwrap();
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mn_src_path = Path::new(&crate_dir).join("MeteoNook/src");
    let dest_path = Path::new(&out_dir);

    // Generate lib.rs and data.rs include
    convert(mn_src_path.join("lib.rs"), Path::new(&out_dir));
    convert(mn_src_path.join("data.rs"), Path::new(&out_dir));

    // Create meteonook.h
    cbindgen::Builder::new()
        .with_src(&dest_path.join("lib.rs"))
	.with_config(cbindgen::Config::from_file(Path::new(&crate_dir).join("cbindgen.toml")).unwrap())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&dest_path.join("../../../meteonook.h"));
}
