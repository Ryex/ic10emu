use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use itertools::Itertools;
use strum::IntoEnumIterator;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("ts_types.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    let mut ts_types: String = String::new();

    let lt_tsunion: String = Itertools::intersperse(
        ic10emu::grammar::generated::LogicType::iter().map(|lt| format!("\"{}\"", lt.as_ref())),
        "\n  | ".to_owned(),
    )
    .collect();
    let lt_tstype = format!("\nexport type LogicType = {};", lt_tsunion);
    ts_types.push_str(&lt_tstype);

    let slt_tsunion: String = Itertools::intersperse(
        ic10emu::grammar::generated::LogicSlotType::iter()
            .map(|slt| format!("\"{}\"", slt.as_ref())),
        "\n  | ".to_owned(),
    )
    .collect();
    let slt_tstype = format!("\nexport type LogicSlotType = {};", slt_tsunion);
    ts_types.push_str(&slt_tstype);

    let bm_tsunion: String = Itertools::intersperse(
        ic10emu::grammar::generated::BatchMode::iter().map(|bm| format!("\"{}\"", bm.as_ref())),
        "\n  | ".to_owned(),
    )
    .collect();
    let bm_tstype = format!("\nexport type BatchMode = {};", bm_tsunion);
    ts_types.push_str(&bm_tstype);

    let rm_tsunion: String = Itertools::intersperse(
        ic10emu::grammar::generated::ReagentMode::iter().map(|rm| format!("\"{}\"", rm.as_ref())),
        "\n  | ".to_owned(),
    )
    .collect();
    let rm_tstype = format!("\nexport type ReagentMode = {};", rm_tsunion);
    ts_types.push_str(&rm_tstype);

    let sc_tsunion: String = Itertools::intersperse(
        ic10emu::device::SortingClass::iter().map(|rm| format!("\"{}\"", rm.as_ref())),
        "\n  | ".to_owned(),
    )
    .collect();
    let sc_tstype = format!("\nexport type SortingClass = {};", sc_tsunion);
    ts_types.push_str(&sc_tstype);

    let st_tsunion: String = Itertools::intersperse(
        ic10emu::device::SlotType::iter().map(|rm| format!("\"{}\"", rm.as_ref())),
        "\n  | ".to_owned(),
    )
    .collect();
    let st_tstype = format!("\nexport type SlotType = {};", st_tsunion);
    ts_types.push_str(&st_tstype);

    let ct_tsunion: String = Itertools::intersperse(
        ic10emu::network::ConnectionType::iter().map(|rm| format!("\"{}\"", rm.as_ref())),
        "\n  | ".to_owned(),
    )
    .collect();
    let ct_tstype = format!("\nexport type ConnectionType = {};", ct_tsunion);
    ts_types.push_str(&ct_tstype);

    let cr_tsunion: String = Itertools::intersperse(
        ic10emu::network::ConnectionRole::iter().map(|rm| format!("\"{}\"", rm.as_ref())),
        "\n  | ".to_owned(),
    )
    .collect();
    let cr_tstype = format!("\nexport type ConnectionRole = {};", cr_tsunion);
    ts_types.push_str(&cr_tstype);

    let infile = Path::new("src/types.ts");
    let contents = fs::read_to_string(infile).unwrap();

    ts_types.push('\n');
    ts_types.push_str(&contents);

    write!(
        &mut writer,
        "#[wasm_bindgen(typescript_custom_section)]\n\
        const TYPES: &'static str = r#\"{ts_types}\"#;
        "
    )
    .unwrap();
}
