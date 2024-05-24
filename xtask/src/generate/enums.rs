use convert_case::{Case, Casing};
use std::collections::BTreeMap;
use std::{
    fmt::Display,
    io::{BufWriter, Write},
    path::PathBuf,
    str::FromStr,
};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn generate(
    stationpedia: &crate::stationpedia::Stationpedia,
    enums: &crate::enums::Enums,
    workspace: &std::path::Path,
) -> color_eyre::Result<Vec<PathBuf>> {
    println!("Writing Enum Listings ...");
    let enums_path = workspace.join("stationeers_data").join("src").join("enums");
    if !enums_path.exists() {
        std::fs::create_dir(&enums_path)?;
    }

    let basic_enum_names = enums
        .basic_enums
        .values()
        .map(|enm| enm.enum_name.clone())
        .collect::<Vec<_>>();

    let mut writer =
        std::io::BufWriter::new(std::fs::File::create(enums_path.join("script.rs"))?);
    write_repr_enum_use_header(&mut writer)?;
    for enm in enums.script_enums.values() {
        write_enum_listing(&mut writer, enm)?;
    }

    let mut writer =
        std::io::BufWriter::new(std::fs::File::create(enums_path.join("basic.rs"))?);
    write_repr_enum_use_header(&mut writer)?;
    let script_enums_in_basic = enums
        .script_enums
        .values()
        .filter(|enm| basic_enum_names.contains(&enm.enum_name))
        .collect::<Vec<_>>();
    let script_enums_in_basic_names = script_enums_in_basic
        .iter()
        .map(|enm| enm.enum_name.as_str())
        .collect::<Vec<_>>();
    write_repr_basic_use_header(&mut writer, script_enums_in_basic.as_slice())?;
    for enm in enums.basic_enums.values() {
        if script_enums_in_basic_names.contains(&enm.enum_name.as_str()) {
            continue;
        }
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
        enums_path.join("script.rs"),
        enums_path.join("basic.rs"),
        enums_path.join("prefabs.rs"),
    ])
}

#[allow(clippy::type_complexity)]
fn write_enum_aggragate_mod<T: std::io::Write>(
    writer: &mut BufWriter<T>,
    enums: &BTreeMap<String, crate::enums::EnumListing>,
) -> color_eyre::Result<()> {
    let (
        (variant_lines, value_arms),
        (
            (get_str_arms, iter_chain),
            (from_str_arms_iter, display_arms)
        )
    ): (
        (Vec<_>, Vec<_>),
        ((Vec<_>, Vec<_>), (Vec<_>, Vec<_>)),
    ) = enums
        .iter()
        .enumerate()
        .map(|(index, (name, listing))| {
            let variant_name: TokenStream = if name.is_empty() || name == "_unnamed" {
                "Unnamed"
            } else {
                name
            }
            .to_case(Case::Pascal)
            .parse()
            .unwrap();
            let fromstr_variant_name = variant_name.clone();
            let enum_name: TokenStream = listing.enum_name.to_case(Case::Pascal).parse().unwrap();
            let display_sep = if name.is_empty() || name == "_unnamed" {
                ""
            } else {
                "."
            };
            let display_pat = format!("{name}{display_sep}{{}}");
            let name: TokenStream = if name == "_unnamed" {
                String::new()
            } else {
                name.clone()
            }
            .parse()
            .unwrap();
            (
                (
                    quote! {
                        #variant_name(#enum_name),
                    },
                    quote! {
                        Self::#variant_name(enm) => *enm as u32,
                    },
                ),
                (
                    (
                        quote! {
                            Self::#variant_name(enm) => enm.get_str(prop),
                        },
                        if index == 0 {
                            quote! {
                                #enum_name::iter().map(Self::#variant_name)
                            }
                        } else {
                            quote! {
                                .chain(#enum_name::iter().map(Self::#variant_name))
                            }
                        },
                    ),
                    (
                        listing.values.keys().map(move |variant| {
                            let sep = if name.is_empty() { "" } else { "." };
                            let fromstr_pat = format!("{name}{sep}{variant}").to_lowercase();
                            let variant: TokenStream = variant.to_case(Case::Pascal).parse().unwrap();
                            quote! {
                                #fromstr_pat => Ok(Self::#fromstr_variant_name(#enum_name::#variant)),
                            }
                        }),
                        quote! {
                            Self::#variant_name(enm) => write!(f, #display_pat, enm),
                        },
                    ),
                ),
            )
        })
        .unzip();

    let from_str_arms = from_str_arms_iter.into_iter().flatten().collect::<Vec<_>>();

    let tokens = quote! {
        pub enum BasicEnum {
            #(#variant_lines)*
        }

        impl BasicEnum {
            pub fn get_value(&self) -> u32 {
                match self {
                    #(#value_arms)*
                }
            }
            pub fn get_str(&self, prop: &str) -> Option<&'static str> {
                match self {
                    #(#get_str_arms)*
                }
            }
            pub fn iter() -> impl std::iter::Iterator<Item = Self> {
                use strum::IntoEnumIterator;
                #(#iter_chain)*
            }
        }

        impl std::str::FromStr for BasicEnum {
            type Err = super::ParseError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_lowercase().as_str() {
                    #(#from_str_arms)*
                    _ => Err(super::ParseError { enm: s.to_string() })
                }
            }
        }

        impl std::fmt::Display for BasicEnum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms)*
                }
            }
        }

    };
    write!(writer, "{tokens}",)?;
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
                    props: vec![("docs".to_owned(), var.description.clone())],
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
                    props: vec![("docs".to_owned(), var.description.clone())],
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
                    props: vec![("docs".to_owned(), var.description.clone())],
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
                    props: vec![("docs".to_owned(), var.description.clone())],
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
                    props: vec![("docs".to_owned(), var.description.clone())],
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
    P: Display + FromStr + Ord,
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
        "{}",
        quote! {
            use serde_derive::{{Deserialize, Serialize}};
            use strum::{
                AsRefStr, Display, EnumIter, EnumProperty, EnumString, FromRepr,
            };
        }
    )?;
    Ok(())
}

fn write_repr_basic_use_header<T: std::io::Write>(
    writer: &mut BufWriter<T>,
    script_enums: &[&crate::enums::EnumListing],
) -> color_eyre::Result<()> {
    let enums = script_enums
        .iter()
        .map(|enm| Ident::new(&enm.enum_name.to_case(Case::Pascal), Span::call_site()))
        .collect::<Vec<_>>();

    write!(
        writer,
        "{}",
        quote! {use super::script::{ #(#enums),*};},
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
    P: Display + FromStr + num::integer::Integer + num::cast::AsPrimitive<i64> + 'a,
    I: IntoIterator<Item = &'a (String, ReprEnumVariant<P>)>,
{
    let additional_strum = if use_phf {
        quote! {#[strum(use_phf)]}
    } else {
        TokenStream::new()
    };
    let repr = Ident::new(std::any::type_name::<P>(), Span::call_site());
    let mut sorted: Vec<_> = variants.into_iter().collect::<Vec<_>>();
    sorted.sort_by_key(|(_, variant)| &variant.value);
    let mut derives = [
        "Debug",
        "Display",
        "Clone",
        "Copy",
        "PartialEq",
        "Eq",
        "PartialOrd",
        "Ord",
        "Hash",
        "EnumString",
        "AsRefStr",
        "EnumProperty",
        "EnumIter",
        "FromRepr",
        "Serialize",
        "Deserialize",
    ]
    .into_iter()
    .map(|d| Ident::new(d, Span::call_site()))
    .collect::<Vec<_>>();
    if sorted
        .iter()
        .any(|(name, _)| name == "None" || name == "Default")
    {
        derives.insert(0, Ident::new("Default", Span::call_site()));
    }

    let variants = sorted
        .iter()
        .map(|(name, variant)| {
            let variant_name = Ident::new(
                &name.replace('.', "").to_case(Case::Pascal),
                Span::call_site(),
            );
            let mut props = Vec::new();
            if variant.deprecated {
                props.push(quote! {deprecated = "true"});
            }
            for (prop_name, prop_val) in &variant.props {
                let prop_name = Ident::new(prop_name, Span::call_site());
                let val_string = prop_val.to_string();
                props.push(quote! { #prop_name = #val_string });
            }
            let val: TokenStream = format!("{}{repr}", variant.value).parse().unwrap();
            let val_string = variant.value.as_().to_string();
            props.push(quote! {value = #val_string });
            let default = if variant_name == "None" || variant_name == "Default" {
                quote! {#[default]}
            } else {
                TokenStream::new()
            };
            quote! {
                #[strum(serialize = #name)]
                #[strum(props(#(#props),*))]
                #default
                #variant_name = #val,
            }
        })
        .collect::<Vec<_>>();
    let name = Ident::new(name, Span::call_site());

    write!(
        writer,
        "{}",
        quote! {
            #[derive(#(#derives),*)]
            #additional_strum
            #[repr(#repr)]
            pub enum #name {
                #(#variants)*
            }

            impl TryFrom<f64> for #name {
                type Error = super::ParseError;
                fn try_from(value: f64) -> Result<Self, <#name as TryFrom<f64>>::Error> {
                    use strum::IntoEnumIterator;
                    if let Some(enm) = #name::iter().find(|enm| (f64::from(*enm as #repr) - value).abs() < f64::EPSILON ) {
                        Ok(enm)
                    } else {
                        Err(super::ParseError {
                            enm: value.to_string()
                        })
                    }
                }
            }
        }
    )?;
    Ok(())
}
