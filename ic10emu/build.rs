use convert_case::{Case, Casing};
use std::{
    collections::BTreeSet,
    env,
    fmt::Display,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
    str::FromStr,
};

struct EnumVariant<P>
where
    P: Display + FromStr,
{
    pub aliases: Vec<String>,
    pub value: Option<P>,
    pub deprecated: bool,
}

fn write_repr_enum<'a, T: std::io::Write, I, P>(
    writer: &mut BufWriter<T>,
    name: &str,
    variants: I,
    use_phf: bool,
) where
    P: Display + FromStr + 'a,
    I: IntoIterator<Item = &'a (String, EnumVariant<P>)>,
{
    let additional_strum = if use_phf { "#[strum(use_phf)]\n" } else { "" };
    write!(
        writer,
         "#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, EnumProperty, EnumIter, Serialize, Deserialize)]\n\
         {additional_strum}\
         pub enum {name} {{\n"
    )
    .unwrap();
    for (name, variant) in variants {
        let variant_name = name.replace('.', "").to_case(Case::Pascal);
        let mut serialize = vec![name.clone()];
        serialize.extend(variant.aliases.iter().cloned());
        let serialize_str = serialize
            .into_iter()
            .map(|s| format!("serialize = \"{s}\""))
            .collect::<Vec<String>>()
            .join(", ");
        let mut props = Vec::new();
        if variant.deprecated {
            props.push("deprecated = \"true\"".to_owned());
        }
        if let Some(val) = &variant.value {
            props.push(format!("value = \"{val}\""));
        }
        let props_str = if !props.is_empty() {
            format!(", props( {} )", props.join(", "))
        } else {
            "".to_owned()
        };
        writeln!(
            writer,
            "    #[strum({serialize_str}{props_str})] {variant_name},"
        )
        .unwrap();
    }
    writeln!(writer, "}}").unwrap();
}

fn write_logictypes() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("logictypes.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    let mut logictypes: Vec<(String, EnumVariant<u16>)> = Vec::new();
    let l_infile = Path::new("data/logictypes.txt");
    let l_contents = fs::read_to_string(l_infile).unwrap();

    for line in l_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u16> = val_str.parse().ok();
        let docs = it.next();
        let deprecated = docs
            .map(|docs| docs.trim().to_uppercase() == "DEPRECATED")
            .unwrap_or(false);

        if let Some(val) = val {
            if let Some((_other_name, variant)) = logictypes
                .iter_mut()
                .find(|(_, variant)| variant.value == Some(val))
            {
                variant.aliases.push(name.to_string());
                variant.deprecated = deprecated;
            } else {
                logictypes.push((
                    name.to_string(),
                    EnumVariant {
                        aliases: Vec::new(),
                        value: Some(val),
                        deprecated,
                    },
                ));
            }
        } else {
            logictypes.push((
                name.to_string(),
                EnumVariant {
                    aliases: Vec::new(),
                    value: val,
                    deprecated,
                },
            ));
        }
    }

    let mut slotlogictypes: Vec<(String, EnumVariant<u8>)> = Vec::new();
    let sl_infile = Path::new("data/slotlogictypes.txt");
    let sl_contents = fs::read_to_string(sl_infile).unwrap();

    for line in sl_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u8> = val_str.parse().ok();
        let docs = it.next();
        let deprecated = docs
            .map(|docs| docs.trim().to_uppercase() == "DEPRECATED")
            .unwrap_or(false);

        if let Some(val) = val {
            if let Some((_other_name, variant)) = slotlogictypes
                .iter_mut()
                .find(|(_, variant)| variant.value == Some(val))
            {
                variant.aliases.push(name.to_string());
                variant.deprecated = deprecated;
            } else {
                slotlogictypes.push((
                    name.to_string(),
                    EnumVariant {
                        aliases: Vec::new(),
                        value: Some(val),
                        deprecated,
                    },
                ));
            }
        } else {
            slotlogictypes.push((
                name.to_string(),
                EnumVariant {
                    aliases: Vec::new(),
                    value: val,
                    deprecated,
                },
            ));
        }
    }

    write_repr_enum(&mut writer, "LogicType", &logictypes, true);

    println!("cargo:rerun-if-changed=data/logictypes.txt");

    write_repr_enum(&mut writer, "SlotLogicType", &slotlogictypes, true);

    println!("cargo:rerun-if-changed=data/slotlogictypes.txt");
}

fn write_enums() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("enums.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    let mut enums_map: Vec<(String, EnumVariant<u32>)> = Vec::new();
    let e_infile = Path::new("data/enums.txt");
    let e_contents = fs::read_to_string(e_infile).unwrap();

    for line in e_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u32> = val_str.parse().ok();
        let docs = it.next();
        let deprecated = docs
            .map(|docs| docs.trim().to_uppercase() == "DEPRECATED")
            .unwrap_or(false);

        enums_map.push((
            name.to_string(),
            EnumVariant {
                aliases: Vec::new(),
                value: val,
                deprecated,
            },
        ));
    }

    write_repr_enum(&mut writer, "LogicEnums", &enums_map, true);

    println!("cargo:rerun-if-changed=data/enums.txt");
}

fn write_modes() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("modes.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    let mut batchmodes: Vec<(String, EnumVariant<u8>)> = Vec::new();
    let b_infile = Path::new("data/batchmodes.txt");
    let b_contents = fs::read_to_string(b_infile).unwrap();

    for line in b_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u8> = val_str.parse().ok();

        if let Some(val) = val {
            if let Some((_other_name, variant)) = batchmodes
                .iter_mut()
                .find(|(_, variant)| variant.value == Some(val))
            {
                variant.aliases.push(name.to_string());
            } else {
                batchmodes.push((
                    name.to_string(),
                    EnumVariant {
                        aliases: Vec::new(),
                        value: Some(val),
                        deprecated: false,
                    },
                ));
            }
        } else {
            batchmodes.push((
                name.to_string(),
                EnumVariant {
                    aliases: Vec::new(),
                    value: val,
                    deprecated: false,
                },
            ));
        }
    }

    let mut reagentmodes: Vec<(String, EnumVariant<u8>)> = Vec::new();
    let r_infile = Path::new("data/reagentmodes.txt");
    let r_contents = fs::read_to_string(r_infile).unwrap();

    for line in r_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u8> = val_str.parse().ok();

        if let Some(val) = val {
            if let Some((_other_name, variant)) = reagentmodes
                .iter_mut()
                .find(|(_, variant)| variant.value == Some(val))
            {
                variant.aliases.push(name.to_string());
            } else {
                reagentmodes.push((
                    name.to_string(),
                    EnumVariant {
                        aliases: Vec::new(),
                        value: Some(val),
                        deprecated: false,
                    },
                ));
            }
        } else {
            reagentmodes.push((
                name.to_string(),
                EnumVariant {
                    aliases: Vec::new(),
                    value: val,
                    deprecated: false,
                },
            ));
        }
    }

    write_repr_enum(&mut writer, "BatchMode", &batchmodes, false);

    println!("cargo:rerun-if-changed=data/batchmodes.txt");

    write_repr_enum(&mut writer, "ReagentMode", &reagentmodes, false);

    println!("cargo:rerun-if-changed=data/reagentmodes.txt");
}

fn write_constants() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("constants.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    let mut constants_lookup_map_builder = ::phf_codegen::Map::new();
    let infile = Path::new("data/constants.txt");
    let contents = fs::read_to_string(infile).unwrap();

    for line in contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let constant = it.next().unwrap();

        constants_lookup_map_builder.entry(name, constant);
    }

    writeln!(
        &mut writer,
        "#[allow(clippy::approx_constant)] pub(crate) const CONSTANTS_LOOKUP: phf::Map<&'static str, f64> = {};",
        constants_lookup_map_builder.build()
    )
    .unwrap();
    println!("cargo:rerun-if-changed=data/constants.txt");
}

fn write_instructions_enum() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("instructions.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    let mut instructions = BTreeSet::new();
    let infile = Path::new("data/instructions.txt");
    let contents = fs::read_to_string(infile).unwrap();

    for line in contents.lines() {
        let mut it = line.split(' ');
        let instruction = it.next().unwrap();
        instructions.insert(instruction.to_string());
    }

    write!(
        &mut writer,
        "#[derive(Debug, Display, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]\n\
         pub enum InstructionOp {{\n\
        "
    )
    .unwrap();

    writeln!(&mut writer, "     Nop,").unwrap();

    for typ in &instructions {
        writeln!(&mut writer, "     {},", typ.to_case(Case::Pascal)).unwrap();
    }
    writeln!(&mut writer, "}}").unwrap();

    write!(
        &mut writer,
        "impl FromStr for InstructionOp {{\n    \
            type Err = ParseError;\n    \
            fn from_str(s: &str) -> Result<Self, Self::Err> {{\n        \
                let end = s.len();\n        \
                match s {{\n"
    )
    .unwrap();

    for typ in &instructions {
        let name = typ.to_case(Case::Pascal);
        writeln!(&mut writer, "            \"{typ}\" => Ok(Self::{name}),").unwrap();
    }
    write!(
        &mut writer,
        "            _ =>    Err(crate::grammar::ParseError {{ line: 0, start: 0, end, msg: format!(\"Unknown instruction '{{}}'\", s) }})\n        \
            }}\n    \
         }}\n\
    }}"
    )
    .unwrap();

    println!("cargo:rerun-if-changed=data/instructions.txt");
}

fn main() {
    // write_instructions();
    write_logictypes();
    write_modes();
    write_constants();
    write_enums();

    write_instructions_enum();
}
