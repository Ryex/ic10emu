use convert_case::{Case, Casing};
use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::Display,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
    str::FromStr,
};

trait PrimitiveRepr {}
impl PrimitiveRepr for u8 {}
impl PrimitiveRepr for u16 {}
impl PrimitiveRepr for u32 {}
impl PrimitiveRepr for u64 {}
impl PrimitiveRepr for u128 {}
impl PrimitiveRepr for usize {}
impl PrimitiveRepr for i8 {}
impl PrimitiveRepr for i16 {}
impl PrimitiveRepr for i32 {}
impl PrimitiveRepr for i64 {}
impl PrimitiveRepr for i128 {}
impl PrimitiveRepr for isize {}

struct EnumVariant<P>
where
    P: Display + FromStr,
{
    pub aliases: Vec<String>,
    pub value: Option<P>,
    pub depricated: bool,
}

fn write_repr_enum<T: std::io::Write, I, P>(
    writer: &mut BufWriter<T>,
    name: &str,
    variants: &I,
    use_phf: bool,
) where
    P: Display + FromStr,
    for<'a> &'a I: IntoIterator<Item = (&'a String, &'a EnumVariant<P>)>,
{
    let additional_strum = if use_phf { "#[strum(use_phf)]\n" } else { "" };
    write!(
        writer,
         "#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, EnumProperty, EnumIter, Serialize, Deserialize)]\n\
         {additional_strum}\
         pub enum {name} {{\n"
    )
    .unwrap();
    for (name, variant) in variants.into_iter() {
        let variant_name = name.to_case(Case::Pascal);
        let mut serialize = vec![name.clone()];
        serialize.extend(variant.aliases.iter().cloned());
        let serialize_str = serialize
            .into_iter()
            .map(|s| format!("serialize = \"{s}\""))
            .collect::<Vec<String>>()
            .join(", ");
        let depricated_str = if variant.depricated {
            ", depricated = \"true\"".to_string()
        } else {
            "".to_string()
        };
        let props_str = if let Some(val) = &variant.value {
            format!(", props( value = \"{val}\"{depricated_str})")
        } else {
            "".to_string()
        };
        write!(
            writer,
            "    #[strum({serialize_str}{props_str})] {variant_name},\n"
        )
        .unwrap();
    }
    write!(writer, "}}\n").unwrap();
}

fn write_logictypes() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("logictypes.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    let mut logictypes: HashMap<String, EnumVariant<u8>> = HashMap::new();
    let l_infile = Path::new("data/logictypes.txt");
    let l_contents = fs::read_to_string(l_infile).unwrap();

    for line in l_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u8> = val_str.parse().ok();
        let docs = it.next();
        let depricated = docs
            .map(|docs| docs.trim().to_uppercase() == "DEPRECATED")
            .unwrap_or(false);

        if let Some(val) = val {
            if let Some((_other_name, variant)) = logictypes
                .iter_mut()
                .find(|(_, variant)| variant.value == Some(val))
            {
                variant.aliases.push(name.to_string());
                variant.depricated = depricated;
            } else {
                logictypes.insert(
                    name.to_string(),
                    EnumVariant {
                        aliases: Vec::new(),
                        value: Some(val),
                        depricated,
                    },
                );
            }
        } else {
            logictypes.insert(
                name.to_string(),
                EnumVariant {
                    aliases: Vec::new(),
                    value: val,
                    depricated,
                },
            );
        }
    }

    let mut slotlogictypes: HashMap<String, EnumVariant<u8>> = HashMap::new();
    let sl_infile = Path::new("data/slotlogictypes.txt");
    let sl_contents = fs::read_to_string(sl_infile).unwrap();

    for line in sl_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u8> = val_str.parse().ok();
        let docs = it.next();
        let depricated = docs
            .map(|docs| docs.trim().to_uppercase() == "DEPRECATED")
            .unwrap_or(false);

        if let Some(val) = val {
            if let Some((_other_name, variant)) = slotlogictypes
                .iter_mut()
                .find(|(_, variant)| variant.value == Some(val))
            {
                variant.aliases.push(name.to_string());
                variant.depricated = depricated;
            } else {
                slotlogictypes.insert(
                    name.to_string(),
                    EnumVariant {
                        aliases: Vec::new(),
                        value: Some(val),
                        depricated,
                    },
                );
            }
        } else {
            slotlogictypes.insert(
                name.to_string(),
                EnumVariant {
                    aliases: Vec::new(),
                    value: val,
                    depricated,
                },
            );
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

    let mut enums_lookup_map_builder = ::phf_codegen::Map::new();
    let mut check_set = std::collections::HashSet::new();
    let e_infile = Path::new("data/enums.txt");
    let e_contents = fs::read_to_string(e_infile).unwrap();

    for line in e_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(2, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u8> = val_str.parse().ok();

        if !check_set.contains(name) {
            check_set.insert(name);
        }

        if let Some(v) = val {
            enums_lookup_map_builder.entry(name, &format!("{}u8", v));
        }
    }

    write!(
        &mut writer,
        "pub(crate) const ENUM_LOOKUP: phf::Map<&'static str, u8> = {};\n",
        enums_lookup_map_builder.build()
    )
    .unwrap();

    println!("cargo:rerun-if-changed=data/enums.txt");
}

fn write_modes() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("modes.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    let mut batchmodes: HashMap<String, EnumVariant<u8>> = HashMap::new();
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
                batchmodes.insert(
                    name.to_string(),
                    EnumVariant {
                        aliases: Vec::new(),
                        value: Some(val),
                        depricated: false,
                    },
                );
            }
        } else {
            batchmodes.insert(
                name.to_string(),
                EnumVariant {
                    aliases: Vec::new(),
                    value: val,
                    depricated: false,
                },
            );
        }
    }

    let mut reagentmodes: HashMap<String, EnumVariant<u8>> = HashMap::new();
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
                reagentmodes.insert(
                    name.to_string(),
                    EnumVariant {
                        aliases: Vec::new(),
                        value: Some(val),
                        depricated: false,
                    },
                );
            }
        } else {
            reagentmodes.insert(
                name.to_string(),
                EnumVariant {
                    aliases: Vec::new(),
                    value: val,
                    depricated: false,
                },
            );
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

    write!(
        &mut writer,
        "pub(crate) const CONSTANTS_LOOKUP: phf::Map<&'static str, f64> = {};\n",
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

    let mut instructions = HashSet::new();
    let infile = Path::new("data/instructions.txt");
    let contents = fs::read_to_string(infile).unwrap();

    for line in contents.lines() {
        let mut it = line.split(' ');
        let instruction = it.next().unwrap();
        instructions.insert(instruction.to_string());
    }

    write!(
        &mut writer,
        "#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]\n\
         pub enum InstructionOp {{\n\
        "
    )
    .unwrap();

    write!(&mut writer, "     Nop,\n").unwrap();

    for typ in &instructions {
        write!(&mut writer, "     {},\n", typ.to_case(Case::Pascal)).unwrap();
    }
    write!(&mut writer, "}}\n").unwrap();

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
        write!(&mut writer, "            \"{typ}\" => Ok(Self::{name}),\n").unwrap();
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
