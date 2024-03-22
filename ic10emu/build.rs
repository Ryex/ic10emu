use convert_case::{Case, Casing};
use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

fn write_logictypes(logictypes_grammar: &mut HashSet<String>) {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("logictypes.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    let mut logictype_lookup_map_builder = ::phf_codegen::Map::new();
    let l_infile = Path::new("data/logictypes.txt");
    let l_contents = fs::read_to_string(l_infile).unwrap();

    for line in l_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u8> = val_str.parse().ok();

        logictypes_grammar.insert(name.to_string());
        if let Some(v) = val {
            logictype_lookup_map_builder.entry(name.to_case(Case::Pascal), &format!("{}u8", v));
        }
    }

    let mut slotlogictype_lookup_map_builder = ::phf_codegen::Map::new();
    let sl_infile = Path::new("data/slotlogictypes.txt");
    let sl_contents = fs::read_to_string(sl_infile).unwrap();

    for line in sl_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u8> = val_str.parse().ok();

        logictypes_grammar.insert(name.to_string());
        if let Some(v) = val {
            slotlogictype_lookup_map_builder.entry(name.to_case(Case::Pascal), &format!("{}u8", v));
        }
    }

    write!(
        &mut writer,
        "pub(crate) const LOGIC_TYPE_LOOKUP: phf::Map<&'static str, u8> = {};\n",
        logictype_lookup_map_builder.build()
    )
    .unwrap();

    println!("cargo:rerun-if-changed=data/logictypes.txt");


    write!(
        &mut writer,
        "pub(crate) const SLOT_TYPE_LOOKUP: phf::Map<&'static str, u8> = {};\n",
        slotlogictype_lookup_map_builder.build()
    )
    .unwrap();

    println!("cargo:rerun-if-changed=data/slotlogictypes.txt");
}

fn write_enums(enums_grammar: &mut HashSet<String>) {
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
            enums_grammar.insert(name.to_string());
            check_set.insert(name);
        }

        if let Some(v) = val {
            enums_lookup_map_builder.entry(name.to_case(Case::Pascal), &format!("{}u8", v));
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

fn write_modes(logictypes_grammar: &mut HashSet<String>) {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("modes.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    let mut batchmode_lookup_map_builder = ::phf_codegen::Map::new();
    let b_infile = Path::new("data/batchmodes.txt");
    let b_contents = fs::read_to_string(b_infile).unwrap();

    for line in b_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u8> = val_str.parse().ok();

        logictypes_grammar.insert(name.to_string());
        if let Some(v) = val {
            batchmode_lookup_map_builder.entry(name.to_case(Case::Pascal), &format!("{}u8", v));
        }
    }

    let mut reagentmode_lookup_map_builder = ::phf_codegen::Map::new();
    let r_infile = Path::new("data/reagentmodes.txt");
    let r_contents = fs::read_to_string(r_infile).unwrap();

    for line in r_contents.lines().filter(|l| !l.trim().is_empty()) {
        let mut it = line.splitn(3, ' ');
        let name = it.next().unwrap();
        let val_str = it.next().unwrap();
        let val: Option<u8> = val_str.parse().ok();

        logictypes_grammar.insert(name.to_string());
        if let Some(v) = val {
            reagentmode_lookup_map_builder.entry(name.to_case(Case::Pascal), &format!("{}u8", v));
        }
    }

    write!(
        &mut writer,
        "pub(crate) const BATCH_MODE_LOOKUP: phf::Map<&'static str, u8> = {};\n",
        batchmode_lookup_map_builder.build()
    )
    .unwrap();

    println!("cargo:rerun-if-changed=data/batchmodes.txt");

    write!(
        &mut writer,
        "pub(crate) const REAGENT_MODE_LOOKUP: phf::Map<&'static str, u8> = {};\n",
        reagentmode_lookup_map_builder.build()
    )
    .unwrap();

    println!("cargo:rerun-if-changed=data/reagentmodes.txt");
}

fn write_constants(constants_grammar: &mut HashSet<String>) {
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

        constants_grammar.insert(name.to_string());
        constants_lookup_map_builder.entry(name.to_case(Case::Pascal), constant);
    }

    write!(
        &mut writer,
        "pub(crate) const CONSTANTS_LOOKUP: phf::Map<&'static str, f64> = {};\n",
        constants_lookup_map_builder.build()
    )
    .unwrap();
    println!("cargo:rerun-if-changed=data/constants.txt");
}

fn write_logictypes_grammar(logictypes: &HashSet<String>) {
    let dest_path = Path::new("src/grammar/ic10").join("logictypes.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);
    write!(
        &mut writer,
        "// GENERATED CODE DO NOT MODIFY\n\
         #[derive(PartialEq, Debug, IntoStaticStr, AsRefStr)]\n\
         pub enum LogicType {{\n\
        "
    )
    .unwrap();
    for typ in logictypes {
        let enum_name = typ.to_case(Case::Pascal);

        write!(
            &mut writer,
            "     #[rust_sitter::leaf(text = \"{typ}\" )]{},\n",
            &enum_name
        )
        .unwrap();
    }
    write!(&mut writer, "}}\n").unwrap();
}

fn write_enums_grammar(enums: &HashSet<String>) {
    let dest_path = Path::new("src/grammar/ic10").join("enums.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);
    write!(
        &mut writer,
        "// GENERATED CODE DO NOT MODIFY\n\
         #[derive(PartialEq, Debug, IntoStaticStr, AsRefStr)]\n\
         pub enum Enum {{\n\
        "
    )
    .unwrap();
    for typ in enums {
        let enum_name = typ.replace(".", "").to_case(Case::Pascal);

        write!(
            &mut writer,
            "     #[rust_sitter::leaf(text = \"{typ}\" )]{},\n",
            &enum_name
        )
        .unwrap();
    }
    write!(&mut writer, "}}\n").unwrap();
}

fn write_constants_grammar(constants: HashSet<String>) {
    let dest_path = Path::new("src/grammar/ic10").join("constants.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);
    write!(
        &mut writer,
        "// GENERATED CODE DO NOT MODIFY\n\
         #[derive(PartialEq, Debug, IntoStaticStr, AsRefStr)]\n\
         pub enum Constant {{\n\
        "
    )
    .unwrap();
    for typ in constants {
        let enum_name = typ.replace(".", "").to_case(Case::Pascal);

        write!(
            &mut writer,
            "     #[rust_sitter::leaf(text = \"{typ}\" )]{},\n",
            &enum_name
        )
        .unwrap();
    }
    write!(&mut writer, "}}\n").unwrap();
}

fn load_instructions() -> HashSet<String> {
    let mut instructions = HashSet::new();
    let infile = Path::new("data/instructions.txt");
    let contents = fs::read_to_string(infile).unwrap();

    for line in contents.lines() {
        let mut it = line.split(' ');
        let instruction = it.next().unwrap();
        instructions.insert(instruction.to_string());
    }
    instructions
}

fn write_instructions_grammar(instructions: &HashSet<String>) {
    let dest_path = Path::new("src/grammar/ic10").join("instructions.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    write!(
        &mut writer,
        "// GENERATED CODE DO NOT MODIFY\n\
         #[derive(PartialEq, Debug, IntoStaticStr, AsRefStr)]\n\
         pub enum InstructionOp {{\n\
        "
    )
    .unwrap();
    for typ in instructions {
        let enum_name = typ.to_case(Case::Pascal);

        write!(
            &mut writer,
            "     #[rust_sitter::leaf(text = \"{typ}\" )]{},\n",
            &enum_name
        )
        .unwrap();
    }
    write!(&mut writer, "}}\n").unwrap();
}

fn patch_grammar() {
    let out_path = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_path);
    let src_dir = Path::new("src");
    fs::copy(
        src_dir.join("grammar.rs"),
        out_dir.join("grammar_unpatched.rs"),
    )
    .unwrap();
    {
        let grammar_file = File::open(src_dir.join("grammar.rs")).unwrap();
        let output_file = File::create(out_dir.join("grammar_patched.rs")).unwrap();
        let mut writer = BufWriter::new(output_file);

        let patch_regex = regex::Regex::new(r"^\s+// PATCH ([\w/.]+)\s*$").unwrap();

        let mut patch_marker: Option<regex::Regex> = None;
        for line in BufReader::new(grammar_file).lines().flatten() {
            if let Some(marker) = &patch_marker {
                if let Some(_) = marker.captures(&line) {
                    write!(&mut writer, "{}\n", line).unwrap();
                    patch_marker = None;
                } else {
                    continue;
                }
            } else {
                if let Some(captures) = patch_regex.captures(&line) {
                    write!(&mut writer, "{}\n", line).unwrap();
                    let in_path = captures.get(1).unwrap();
                    patch_marker = Some(
                        regex::Regex::new(&format!(r"^\s+// END PATCH {}\s*$", in_path.as_str()))
                            .unwrap(),
                    );
                    let in_buff =
                        BufReader::new(File::open(src_dir.join(in_path.as_str())).unwrap());
                    write!(
                        &mut writer,
                        "    {}\n",
                        in_buff.lines().flatten().collect::<Vec<_>>().join("\n    ")
                    )
                    .unwrap();
                } else {
                    write!(&mut writer, "{}\n", line).unwrap();
                }
            }
        }
    }
    fs::rename(
        out_dir.join("grammar_patched.rs"),
        src_dir.join("grammar.rs"),
    )
    .unwrap();
}

fn build_grammar() {
    println!("cargo:rerun-if-changed=src/grammar/");
    println!("cargo:rerun-if-changed=src/grammar.rs");
    // let out_path = env::var_os("OUT_DIR").unwrap();
    // let out_dir = Path::new(&out_path);
    rust_sitter_tool::build_parsers(&PathBuf::from("src/lib.rs"));
    // rust_sitter_tool::build_parsers(&out_dir.join("grammar_patched.rs"));
}

fn write_instructions_enum(instructions: &HashSet<String>) {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("instructions.rs");
    let output_file = File::create(dest_path).unwrap();
    let mut writer = BufWriter::new(&output_file);

    write!(
        &mut writer,
        "#[derive(PartialEq, Debug)]\n\
         pub enum InstructionOp {{\n\
        "
    )
    .unwrap();

    write!(&mut writer, "     Nop,\n").unwrap();

    for typ in instructions {
        write!(&mut writer, "     {},\n", typ.to_case(Case::Pascal)).unwrap();
    }
    write!(&mut writer, "}}\n").unwrap();

    write!(
        &mut writer,
        "impl From<crate::grammar::ic10::InstructionOp> for InstructionOp {{\n    \
        fn from(value: crate::grammar::ic10::InstructionOp) -> Self {{\n        \
            match value {{\n"
    )
    .unwrap();

    for typ in instructions {
        let name = typ.to_case(Case::Pascal);
        write!(
            &mut writer,
            "            crate::grammar::ic10::InstructionOp::{name} => Self::{name},\n"
        )
        .unwrap();
    }
    write!(
        &mut writer,
        "        }}\n    \
         }}\n\
    }}"
    )
    .unwrap();
}

fn main() {
    let mut logictype_grammar = HashSet::new();
    let mut enums_grammar = HashSet::new();
    let mut constants_grammar = HashSet::new();
    // write_instructions();
    write_logictypes(&mut logictype_grammar);
    write_modes(&mut logictype_grammar);
    write_constants(&mut constants_grammar);
    write_enums(&mut enums_grammar);

    let instructions = load_instructions();
    write_instructions_enum(&instructions);

    write_logictypes_grammar(&logictype_grammar);
    write_enums_grammar(&enums_grammar);
    write_constants_grammar(constants_grammar);
    write_instructions_grammar(&instructions);

    patch_grammar();

    build_grammar();
}
