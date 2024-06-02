use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use quote::quote;
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
        .join("instructions")
        .join("codegen");
    if !instructions_path.exists() {
        std::fs::create_dir(&instructions_path)?;
    }
    let mut writer =
        std::io::BufWriter::new(std::fs::File::create(instructions_path.join("enums.rs"))?);
    write_instructions_enum(&mut writer, &stationpedia.script_commands)?;

    let mut writer =
        std::io::BufWriter::new(std::fs::File::create(instructions_path.join("traits.rs"))?);

    write_instruction_trait_use(&mut writer)?;
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
    eprintln!("Writing instruction Listings ...");

    let mut instructions = instructions.clone();
    for (_, ref mut info) in instructions.iter_mut() {
        info.example = utils::strip_color(&info.example);
    }

    write!(
        writer,
        "{}",
        quote::quote! {
            use serde_derive::{Deserialize, Serialize};
            use strum::{
                 Display, EnumIter, EnumProperty, EnumString, FromRepr,
            };
            use crate::vm::object::traits::Programmable;

            #[cfg(feature = "tsify")]
            use tsify::Tsify;
            #[cfg(feature = "tsify")]
            use wasm_bindgen::prelude::*;
        }
    )?;

    let inst_variants = instructions
        .iter()
        .map(|(name, info)| {
            let example = &info.example;
            let desc = &info.desc;
            let op_count = count_operands(&info.example).to_string();
            let props =
                quote::quote! { props( example = #example, desc = #desc, operands = #op_count ) };
            let name = Ident::new(&name.to_case(Case::Pascal), Span::call_site());
            quote::quote! {
                #[strum(#props)] #name,
            }
        })
        .collect::<Vec<_>>();

    write!(
        writer,
        "{}",
        quote::quote! {#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
            #[derive(EnumIter, EnumString, EnumProperty, FromRepr)]
            #[cfg_attr(feature = "tsify", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
            #[strum(use_phf, serialize_all = "lowercase")]
            #[serde(rename_all = "lowercase")]
            pub enum InstructionOp {
                    Nop,
                    #(#inst_variants)*

            }
        }
    )?;

    let exec_arms = instructions
        .iter()
        .map(|(name, info)| {
            let num_operands = count_operands(&info.example);
            let operands = (0..num_operands)
                .map(|i| quote! {&operands[#i]})
                .collect::<Vec<_>>();

            let trait_name = Ident::new(&name.to_case(Case::Pascal), Span::call_site());
            let fn_name = Ident::new(&format!("execute_{name}"), Span::call_site());
            quote! {
                Self::#trait_name => ic.#fn_name(#(#operands),*),
            }
        })
        .collect::<Vec<_>>();

    write!(
        writer,
        "{}",
        quote! {
            impl InstructionOp {
                pub fn num_operands(&self) -> usize {
                    self.get_str("operands")
                        .expect("instruction without operand property")
                        .parse::<usize>()
                        .expect("invalid instruction operand property")
                }

                pub fn execute<T>(
                    &self,
                    ic: &mut T,
                    operands: &[crate::vm::instructions::operands::Operand],
                ) -> Result<(), crate::errors::ICError>
                where
                    T: Programmable,
                {
                    let num_operands = self.num_operands();
                    if operands.len() != num_operands {
                        return Err(crate::errors::ICError::mismatch_operands(operands.len(), num_operands as u32));
                    }
                    match self {
                        Self::Nop => Ok(()),
                        #(#exec_arms)*
                    }
                }
            }
        }
    )?;

    Ok(())
}

fn write_instruction_trait<T: std::io::Write>(
    writer: &mut T,
    instruction: (&str, &stationpedia::Command),
) -> color_eyre::Result<()> {
    let (name, info) = instruction;
    let op_name = name.to_case(Case::Pascal);
    let trait_name = Ident::new(&format!("{op_name}Instruction"), Span::call_site());
    let op_ident = Ident::new(&op_name, Span::call_site());
    let operands = operand_names(&info.example)
        .iter()
        .map(|name| {
            let mut n: &str = name;
            if n == "str" {
                n = "string";
            }
            let n = Ident::new(&n.to_case(Case::Snake), Span::call_site());
            quote! {
                #n: &crate::vm::instructions::operands::Operand
            }
        })
        .collect::<Vec<_>>();

    let operands_inner = operand_names(&info.example)
        .iter()
        .map(|name| {
            let mut n: &str = name;
            if n == "str" {
                n = "string";
            }
            let n = Ident::new(&n.to_case(Case::Snake), Span::call_site());
            quote! {
                #n: &crate::vm::instructions::operands::InstOperand
            }
        })
        .collect::<Vec<_>>();

    let operand_call = operand_names(&info.example)
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let mut n: &str = name;
            if n == "str" {
                n = "string";
            }
            let n = Ident::new(&n.to_case(Case::Snake), Span::call_site());
            quote!{
                &crate::vm::instructions::operands::InstOperand::new(#n, InstructionOp::#op_ident, #index)
            }
        })
        .collect::<Vec<_>>();
    let example = utils::strip_color(&info.example);
    let fn_name = Ident::new(&format!("execute_{name}"), Span::call_site());
    write!(
        writer,
        "{}",
        quote! {
            pub trait #trait_name: IntegratedCircuit {
                #[doc = #example]
                fn #fn_name(&mut self, #(#operands),*) -> Result<(), crate::errors::ICError> {
                    #trait_name::execute_inner(self, #(#operand_call),*)
                }
                #[doc = #example]
                fn execute_inner(&mut self, #(#operands_inner),*) -> Result<(), crate::errors::ICError>;
            }
        }
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

fn write_instruction_trait_use<T: std::io::Write>(writer: &mut T) -> color_eyre::Result<()> {
    write!(
        writer,
        "{}",
        quote! {
            use crate::vm::object::traits::IntegratedCircuit;
            use crate::vm::instructions::enums::InstructionOp;
        }
    )?;
    Ok(())
}

fn write_instruction_super_trait<T: std::io::Write>(
    writer: &mut T,
    instructions: &BTreeMap<String, stationpedia::Command>,
) -> color_eyre::Result<()> {
    let traits = instructions
        .keys()
        .map(|name| {
            Ident::new(
                &format!("{}Instruction", name.to_case(Case::Pascal)),
                Span::call_site(),
            )
        })
        .collect::<Vec<_>>();
    write!(
        writer,
        "{}",
        quote! {
            pub trait ICInstructable: #(#traits +)* {}
            impl <T> ICInstructable for T where T: #(#traits )+* {}
        }
    )?;
    Ok(())
}
