use convert_case::{Case, Casing};
use std::collections::BTreeMap;
use std::{
    fmt::Display,
    io::{BufWriter, Write},
    path::PathBuf,
    str::FromStr,
};
pub fn generate_enums(
    stationpedia: &crate::stationpedia::Stationpedia,
    enums: &crate::enums::Enums,
    workspace: &std::path::Path,
) -> color_eyre::Result<Vec<PathBuf>> {
    println!("Writing Enum Listings ...");
    let enums_path = workspace
        .join("ic10emu")
        .join("src")
        .join("vm")
        .join("enums");
    if !enums_path.exists() {
        std::fs::create_dir(&enums_path)?;
    }

    let mut writer =
        std::io::BufWriter::new(std::fs::File::create(enums_path.join("script_enums.rs"))?);
    write_repr_enum_use_header(&mut writer)?;
    for enm in enums.script_enums.values() {
        write_enum_listing(&mut writer, enm)?;
    }

    let mut writer =
        std::io::BufWriter::new(std::fs::File::create(enums_path.join("basic_enums.rs"))?);
    write_repr_enum_use_header(&mut writer)?;
    for enm in enums.basic_enums.values() {
        write_enum_listing(&mut writer, enm)?;
    }
    write_enum_aggragate_mod(&mut writer, &enums.basic_enums)?;

    let mut writer = std::io::BufWriter::new(std::fs::File::create(enums_path.join("prefabs.rs"))?);
    write_repr_enum_use_header(&mut writer)?;
    let prefabs = stationpedia
        .pages
        .iter()
        .map(|page| {
            let variant = ReprEnumVariant {
                value: page.prefab_hash,
                deprecated: false,
                props: vec![
                    ("name".to_owned(), page.title.clone()),
                    ("desc".to_owned(), page.description.clone()),
                ],
            };
            (page.prefab_name.clone(), variant)
        })
        .collect::<Vec<_>>();
    write_repr_enum(&mut writer, "StationpediaPrefab", &prefabs, true)?;

    Ok(vec![
        enums_path.join("script_enums.rs"),
        enums_path.join("basic_enums.rs"),
        enums_path.join("prefabs.rs"),
    ])
}

fn write_enum_aggragate_mod<T: std::io::Write>(
    writer: &mut BufWriter<T>,
    enums: &BTreeMap<String, crate::enums::EnumListing>,
) -> color_eyre::Result<()> {
    let variant_lines = enums
        .iter()
        .map(|(name, listing)| {
            let name = if name.is_empty() || name == "_unnamed" {
                "Unnamed"
            } else {
                name
            };
            format!(
                "    {}({}),",
                name.to_case(Case::Pascal),
                listing.enum_name.to_case(Case::Pascal)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let value_arms = enums
        .keys()
        .map(|name| {
            let variant_name = (if name.is_empty() || name == "_unnamed" {
                "Unnamed"
            } else {
                name
            })
            .to_case(Case::Pascal);
            format!("            Self::{variant_name}(enm) => *enm as u32,",)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let get_str_arms = enums
        .keys()
        .map(|name| {
            let variant_name = (if name.is_empty() || name == "_unnamed" {
                "Unnamed"
            } else {
                name
            })
            .to_case(Case::Pascal);
            format!("            Self::{variant_name}(enm) => enm.get_str(prop),",)
        })
        .collect::<Vec<_>>()
        .join("\n");
    let iter_chain = enums
        .iter()
        .enumerate()
        .map(|(index, (name, listing))| {
            let variant_name = (if name.is_empty() || name == "_unnamed" {
                "Unnamed"
            } else {
                name
            })
            .to_case(Case::Pascal);
            let enum_name = listing.enum_name.to_case(Case::Pascal);
            if index == 0 {
                format!("{enum_name}::iter().map(|enm| Self::{variant_name}(enm))")
            } else {
                format!(".chain({enum_name}::iter().map(|enm| Self::{variant_name}(enm)))")
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    write!(
        writer,
        "pub enum BasicEnum {{\n\
            {variant_lines}
        }}\n\
        impl BasicEnum {{\n    \
            pub fn get_value(&self) -> u32 {{\n        \
                match self {{\n            \
                    {value_arms}\n        \
                }}\n    \
            }}\n\
            pub fn get_str(&self, prop: &str) -> Option<&'static str> {{\n        \
                match self {{\n            \
                    {get_str_arms}\n        \
                }}\n    \
            }}\n\
            pub fn iter() -> impl std::iter::Iterator<Item = Self>  {{\n        \
                use strum::IntoEnumIterator;\n        \
                {iter_chain}\n    \
            }}
        }}\n\
        "
    )?;
    let arms = enums
        .iter()
        .flat_map(|(name, listing)| {
            let variant_name = (if name.is_empty() || name == "_unnamed" {
                "Unnamed"
            } else {
                name
            })
            .to_case(Case::Pascal);
            let name = if name == "_unnamed" {
                "".to_string()
            } else {
                name.clone()
            };
            let enum_name = listing.enum_name.to_case(Case::Pascal);
            listing.values.keys().map(move |variant| {
                let sep = if name.is_empty() { "" } else { "." };
                let pat = format!("{name}{sep}{variant}").to_lowercase();
                let variant = variant.to_case(Case::Pascal);
                format!("\"{pat}\" => Ok(Self::{variant_name}({enum_name}::{variant})),")
            })
        })
        .collect::<Vec<_>>()
        .join("\n            ");
    write!(
        writer,
        "\
        impl std::str::FromStr for BasicEnum {{\n    \
            type Err = crate::errors::ParseError;\n    \
            fn from_str(s: &str) -> Result<Self, Self::Err> {{\n        \
                let end = s.len();\n        \
                match s.to_lowercase().as_str() {{\n            \
                    {arms}\n            \
                    _ => Err(crate::errors::ParseError{{ line: 0, start: 0, end, msg: format!(\"Unknown enum '{{}}'\", s) }})\n        \
                }}\n    \
            }}\n\
        }}\
        "
    )?;
    let display_arms = enums
        .keys()
        .map(|name| {
            let variant_name = (if name.is_empty() || name == "_unnamed" {
                "Unnamed"
            } else {
                name
            })
            .to_case(Case::Pascal);
            let name = if name == "_unnamed" {
                "".to_string()
            } else {
                name.clone()
            };
            let sep = if name.is_empty() || name == "_unnamed" {
                ""
            } else {
                "."
            };
            let pat = format!("{name}{sep}{{}}");
            format!("            Self::{variant_name}(enm) => write!(f, \"{pat}\", enm),",)
        })
        .collect::<Vec<_>>()
        .join("\n            ");
    write!(
        writer,
        "\
        impl std::fmt::Display for BasicEnum {{\n    \
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n        \
                match self {{\n            \
                    {display_arms}\n            \
                }}\n    \
            }}\n\
        }}\
        "
    )?;
    Ok(())
}
pub fn write_enum_listing<T: std::io::Write>(
    writer: &mut BufWriter<T>,
    enm: &crate::enums::EnumListing,
) -> color_eyre::Result<()> {
    let max = enm
        .values
        .values()
        .map(|var| var.value)
        .max()
        .expect("enum should have max value");
    let min = enm
        .values
        .values()
        .map(|var| var.value)
        .min()
        .expect("enum should have min value");

    if max < u8::MAX as i64 && min >= u8::MIN as i64 {
        let variants: Vec<_> = enm
            .values
            .iter()
            .map(|(n, var)| {
                let variant = ReprEnumVariant {
                    value: var.value as u8,
                    deprecated: var.deprecated,
                    props: vec![("docs".to_owned(), var.description.to_owned())],
                };
                (n.clone(), variant)
            })
            .collect();
        write_repr_enum(
            writer,
            &enm.enum_name.to_case(Case::Pascal),
            &variants,
            true,
        )?;
    } else if max < u16::MAX as i64 && min >= u16::MIN as i64 {
        let variants: Vec<_> = enm
            .values
            .iter()
            .map(|(n, var)| {
                let variant = ReprEnumVariant {
                    value: var.value as u16,
                    deprecated: var.deprecated,
                    props: vec![("docs".to_owned(), var.description.to_owned())],
                };
                (n.clone(), variant)
            })
            .collect();
        write_repr_enum(writer, &enm.enum_name, &variants, true)?;
    } else if max < u32::MAX as i64 && min >= u32::MIN as i64 {
        let variants: Vec<_> = enm
            .values
            .iter()
            .map(|(n, var)| {
                let variant = ReprEnumVariant {
                    value: var.value as u32,
                    deprecated: var.deprecated,
                    props: vec![("docs".to_owned(), var.description.to_owned())],
                };
                (n.clone(), variant)
            })
            .collect();
        write_repr_enum(writer, &enm.enum_name, &variants, true)?;
    } else if max < i32::MAX as i64 && min >= i32::MIN as i64 {
        let variants: Vec<_> = enm
            .values
            .iter()
            .map(|(n, var)| {
                let variant = ReprEnumVariant {
                    value: var.value as i32,
                    deprecated: var.deprecated,
                    props: vec![("docs".to_owned(), var.description.to_owned())],
                };
                (n.clone(), variant)
            })
            .collect();
        write_repr_enum(writer, &enm.enum_name, &variants, true)?;
    } else {
        let variants: Vec<_> = enm
            .values
            .iter()
            .map(|(n, var)| {
                let variant = ReprEnumVariant {
                    value: var.value as i32,
                    deprecated: var.deprecated,
                    props: vec![("docs".to_owned(), var.description.to_owned())],
                };
                (n.clone(), variant)
            })
            .collect();
        write_repr_enum(writer, &enm.enum_name, &variants, true)?;
    }
    Ok(())
}

struct ReprEnumVariant<P>
where
    P: Display + FromStr,
{
    pub value: P,
    pub deprecated: bool,
    pub props: Vec<(String, String)>,
}

fn write_repr_enum_use_header<T: std::io::Write>(
    writer: &mut BufWriter<T>,
) -> color_eyre::Result<()> {
    write!(
        writer,
        "use serde_derive::{{Deserialize, Serialize}};\n\
          use strum::{{\n    \
              AsRefStr, Display, EnumIter, EnumProperty, EnumString, FromRepr,\n\
          }};\n"
    )?;
    Ok(())
}

fn write_repr_enum<'a, T: std::io::Write, I, P>(
    writer: &mut BufWriter<T>,
    name: &str,
    variants: I,
    use_phf: bool,
) -> color_eyre::Result<()>
where
    P: Display + FromStr + 'a,
    I: IntoIterator<Item = &'a (String, ReprEnumVariant<P>)>,
{
    let additional_strum = if use_phf { "#[strum(use_phf)]\n" } else { "" };
    let repr = std::any::type_name::<P>();
    write!(
        writer,
         "#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString, AsRefStr, EnumProperty, EnumIter, FromRepr, Serialize, Deserialize)]\n\
         {additional_strum}\
         #[repr({repr})]\n\
         pub enum {name} {{\n"
    )?;
    for (name, variant) in variants {
        let variant_name = name.replace('.', "").to_case(Case::Pascal);
        let serialize = vec![name.clone()];
        let serialize_str = serialize
            .into_iter()
            .map(|s| format!("serialize = \"{s}\""))
            .collect::<Vec<String>>()
            .join(", ");
        let mut props = Vec::new();
        if variant.deprecated {
            props.push("deprecated = \"true\"".to_owned());
        }
        for (prop_name, prop_val) in &variant.props {
            props.push(format!("{prop_name} = r#\"{prop_val}\"#"));
        }
        let val = &variant.value;
        props.push(format!("value = \"{val}\""));
        let props_str = if !props.is_empty() {
            format!(", props( {} )", props.join(", "))
        } else {
            "".to_owned()
        };
        writeln!(
            writer,
            "    #[strum({serialize_str}{props_str})] {variant_name} = {val}{repr},"
        )?;
    }
    writeln!(writer, "}}")?;
    Ok(())
}
