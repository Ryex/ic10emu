use convert_case::{Case, Casing};
use std::{collections::BTreeMap, path::PathBuf};

use crate::{generate::utils, stationpedia};

pub fn generate_instructions(
    stationpedia: &stationpedia::Stationpedia,
    workspace: &std::path::Path,
) -> color_eyre::Result<Vec<PathBuf>> {
    let instructions_path = workspace
        .join("ic10emu")
        .join("src")
        .join("vm")
        .join("instructions");
    if !instructions_path.exists() {
        std::fs::create_dir(&instructions_path)?;
    }
    let mut writer =
        std::io::BufWriter::new(std::fs::File::create(instructions_path.join("enums.rs"))?);
    write_instructions_enum(&mut writer, &stationpedia.script_commands)?;

    let mut writer =
        std::io::BufWriter::new(std::fs::File::create(instructions_path.join("traits.rs"))?);

    write_instruction_interface_trait(&mut writer)?;
    for (typ, info) in &stationpedia.script_commands {
        write_instruction_trait(&mut writer, (typ, info))?;
    }
    write_instruction_super_trait(&mut writer, &stationpedia.script_commands)?;

    Ok(vec![
        instructions_path.join("enums.rs"),
        instructions_path.join("traits.rs"),
    ])
}

fn write_instructions_enum<T: std::io::Write>(
    writer: &mut T,
    instructions: &BTreeMap<String, stationpedia::Command>,
) -> color_eyre::Result<()> {
    println!("Writing instruction Listings ...");

    let mut instructions = instructions.clone();
    for (_, ref mut info) in instructions.iter_mut() {
        info.example = utils::strip_color(&info.example);
    }

    write!(
        writer,
        "use serde::{{Deserialize, Serialize}};\n\
         use strum::{{\n    \
              Display, EnumIter, EnumProperty, EnumString, FromRepr,\n\
         }};\n
         use crate::vm::object::traits::Programmable;\n\
         use crate::vm::instructions::traits::*;\n\
        "
    )?;

    write!(
        writer,
        "#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]\n\
         #[derive(EnumIter, EnumString, EnumProperty, FromRepr)]\n\
         #[strum(use_phf, serialize_all = \"lowercase\")]\n\
         #[serde(rename_all = \"lowercase\")]\n\
         pub enum InstructionOp {{\n\
        "
    )?;
    writeln!(writer, "    Nop,")?;
    for (name, info) in &instructions {
        let props_str = format!(
            "props( example = \"{}\", desc = \"{}\", operands = \"{}\" )",
            &info.example,
            &info.desc,
            count_operands(&info.example)
        );
        writeln!(
            writer,
            "    #[strum({props_str})] {},",
            name.to_case(Case::Pascal)
        )?;
    }
    writeln!(writer, "}}")?;

    write!(
        writer,
        "impl InstructionOp {{\n    \
            pub fn num_operands(&self) -> usize {{\n        \
                self.get_str(\"operands\").expect(\"instruction without operand property\").parse::<usize>().expect(\"invalid instruction operand property\")\n    \
            }}\n\
            \n    \
            pub fn execute<T>(\n        \
                &self,\n        \
                ic: &mut T,\n        \
                vm: &crate::vm::VM,\n        \
                operands: &[crate::vm::instructions::operands::Operand],\n        \
            ) -> Result<(), crate::errors::ICError>\n    \
            where\n    \
                T: Programmable,\n\
            {{\n        \
                let num_operands = self.num_operands();\n        \
                if operands.len() != num_operands {{\n            \
                    return Err(crate::errors::ICError::mismatch_operands(operands.len(), num_operands as u32));\n        \
                }}\n        \
                match self {{\n            \
                    Self::Nop => Ok(()),\n            \
        "
    )?;

    for (name, info) in instructions {
        let num_operands = count_operands(&info.example);
        let operands = (0..num_operands)
            .map(|i| format!("&operands[{}]", i))
            .collect::<Vec<_>>()
            .join(", ");
        let trait_name = name.to_case(Case::Pascal);
        writeln!(
            writer,
            "            Self::{trait_name} => ic.execute_{name}(vm, {operands}),",
        )?;
    }

    write!(
        writer,
        "       }}\
            }}\n\
        }}
        "
    )?;

    Ok(())
}

fn write_instruction_trait<T: std::io::Write>(
    writer: &mut T,
    instruction: (&str, &stationpedia::Command),
) -> color_eyre::Result<()> {
    let (name, info) = instruction;
    let trait_name = format!("{}Instruction", name.to_case(Case::Pascal));
    let operands = operand_names(&info.example)
        .iter()
        .map(|name| {
            format!(
                "{}: &crate::vm::instructions::operands::Operand",
                name.to_case(Case::Snake)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let example = utils::strip_color(&info.example);
    write!(
        writer,
        "pub trait {trait_name}: IntegratedCircuit {{\n    \
            /// {example} \n    \
            fn execute_{name}(&mut self, vm: &crate::vm::VM, {operands}) -> Result<(), crate::errors::ICError>;\n\
        }}"
    )?;
    Ok(())
}

fn count_operands(example: &str) -> usize {
    example.split(' ').count() - 1
}

fn operand_names(example: &str) -> Vec<String> {
    utils::strip_color(example)
        .split(' ')
        .skip(1)
        .map(|name| name.split(['?', '(']).next().unwrap().to_string())
        .collect()
}

fn write_instruction_interface_trait<T: std::io::Write>(writer: &mut T) -> color_eyre::Result<()> {
    write!(
        writer,
        "\
        use std::collections::BTreeMap;\n\
        use crate::vm::object::traits::{{Logicable, MemoryWritable, SourceCode}};\n\
        use crate::errors::ICError; \n\
        pub trait IntegratedCircuit: Logicable + MemoryWritable + SourceCode {{\n    \
            fn get_instruciton_pointer(&self) -> usize;\n    \
            fn set_next_instruction(&mut self, next_instruction: usize);\n    \
            fn reset(&mut self);\n    \
            fn get_real_target(&self, indirection: u32, target: u32) -> Result<f64, ICError>;\n    \
            fn get_register(&self, indirection: u32, target: u32) -> Result<f64, ICError>;\n    \
            fn set_register(&mut self, indirection: u32, target: u32, val: f64) -> Result<f64, ICError>;\n    \
            fn set_return_address(&mut self, addr: f64);\n    \
            fn push_stack(&mut self, val: f64) -> Result<f64, ICError>;\n    \
            fn pop_stack(&mut self) -> Result<f64, ICError>;\n    \
            fn peek_stack(&self) -> Result<f64, ICError>;\n    \
            fn get_aliases(&self) -> &BTreeMap<String, crate::vm::instructions::operands::Operand>;\n    \
            fn get_defines(&self) -> &BTreeMap<String, f64>;\n    \
            fn get_lables(&self) -> &BTreeMap<String, u32>;\n\
        }}\n\
        "
    )?;
    Ok(())
}

fn write_instruction_super_trait<T: std::io::Write>(
    writer: &mut T,
    instructions: &BTreeMap<String, stationpedia::Command>,
) -> color_eyre::Result<()> {
    let traits = instructions
        .keys()
        .map(|name| format!("{}Instruction", name.to_case(Case::Pascal)))
        .collect::<Vec<_>>()
        .join(" + ");
    write!(
        writer,
        "\
        pub trait ICInstructable: {traits} {{}}\n\
        impl <T> ICInstructable for T where T: {traits} {{}}
        "
    )?;
    Ok(())
}
