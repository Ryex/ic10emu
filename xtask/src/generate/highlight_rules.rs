use std::collections::BTreeMap;
use std::{
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::enums::EnumEntry;

pub fn generate(
    stationpedia: &crate::stationpedia::Stationpedia,
    enums: &crate::enums::Enums,
    workspace: &std::path::Path,
) -> color_eyre::Result<Vec<(PathBuf, bool)>> {
    let js_path = workspace.join("ic10emu_frontend").join("js");
    if !js_path.exists() {
        std::fs::create_dir(&js_path)?;
    }

    let mut writer =
        std::io::BufWriter::new(std::fs::File::create(js_path.join("ic10_hl_parts.ts"))?);

    let ops = stationpedia
        .script_commands
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();

    write_js_export_const_str(&mut writer, "ops")?;
    write_multiline_js_string(&mut writer, &ops, " ")?;
    writeln!(writer, ";")?;

    let mut deprecated: Vec<&str> = Vec::new();

    for (enum_name, listing) in enums.script_enums.iter() {
        deprecated.extend(
            listing
                .values
                .iter()
                .filter(|(_, entry)| entry.deprecated)
                .map(|(name, _)| name.as_str()),
        );

        let entries: Vec<&str> = listing
            .values
            .iter()
            .filter(|(_, entry)| !entry.deprecated)
            .map(|(name, _)| name.as_str())
            .collect();

        if entries.is_empty() {
            tracing::debug!("empty entries? {enum_name} {listing:?}");
        }

        write_js_export_const_str(&mut writer, &enum_name)?;
        write_multiline_js_string(&mut writer, &entries, " ")?;
        writeln!(writer, ";")?;
    }

    let mut qualified_enums: BTreeMap<String, &EnumEntry> = BTreeMap::new();
    for (enum_name, listing) in enums.basic_enums.iter() {
        let prefix = if enum_name == "_unnamed" {
            "".to_string()
        } else {
            format!("{enum_name}.")
        };

        qualified_enums.extend(
            listing
                .values
                .iter()
                .map(|(name, entry)| (format!("{prefix}{name}"), entry)),
        );
    }

    deprecated.extend(
        qualified_enums
            .iter()
            .filter(|(_, entry)| entry.deprecated)
            .map(|(name, _)| name.as_str()),
    );

    let enum_entries: Vec<&str> = qualified_enums
        .iter()
        .filter(|(_, entry)| !entry.deprecated)
        .map(|(name, _)| name.as_str())
        .collect();

    write_js_export_const_str(&mut writer, "enums")?;
    write_multiline_js_string(&mut writer, &enum_entries, " ")?;
    writeln!(writer, ";")?;

    write_js_export_const_str(&mut writer, "deprecated")?;
    write_multiline_js_string(&mut writer, &deprecated, " ")?;
    writeln!(writer, ";")?;

    let constants: Vec<&str> = stationpedia
        .script_constants
        .keys()
        .map(String::as_str)
        .collect();
    write_js_export_const_str(&mut writer, "constants")?;
    write_multiline_js_string(&mut writer, &constants, " ")?;
    writeln!(writer, ";")?;

    // let basic_enum_names = enums
    //     .basic_enums
    //     .iter()
    //     .map(|(key, enm)| enm.enum_name.clone())
    //     .collect::<Vec<_>>();
    Ok(vec![(js_path.join("ic10_hl_parts.ts"), false)])
}

const MAX_JS_STR_LENGTH: usize = 74;

pub fn write_multiline_js_string<T: std::io::Write>(
    writer: &mut BufWriter<T>,
    items: &[&str],
    sep: &str,
) -> color_eyre::Result<()> {
    let mut lines: Vec<Vec<&str>> = Vec::new();
    let mut line: Vec<&str> = Vec::new();
    let mut line_len = 0;
    for item in items.into_iter() {
        let spaces = if line.len() > 0 { line.len() - 1 } else { 0 };
        if (line_len + spaces + item.len()) > MAX_JS_STR_LENGTH {
            line.push("");
            lines.push(line);
            line = Vec::new();
            line_len = 0;
        }
        line.push(item);
        line_len += item.len();
    }
    lines.push(line);

    writeln!(writer, "(")?;
    for (index, line) in lines.iter().enumerate() {
        let str = line.join(sep);
        let plus = if index < lines.len() - 1 { " +" } else { "" };
        writeln!(writer, r#"    "{str}"{plus}"#)?;
    }
    write!(writer, ")")?;
    Ok(())
}

pub fn write_js_export_const_str<T: std::io::Write>(
    writer: &mut BufWriter<T>,
    name: &str,
) -> color_eyre::Result<()> {
    writeln!(writer, "")?;
    write!(writer, "export const {name} = ")?;
    Ok(())
}
