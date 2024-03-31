use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("ts_types.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    let infile = Path::new("src/types.ts");
    let contents = fs::read_to_string(infile).unwrap();
    write!(
        &mut writer,
        "#[wasm_bindgen(typescript_custom_section)]\n\
        const TYPES: &'static str = r#\"{contents}\"#;
        "
    )
    .unwrap();
}
