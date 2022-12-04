extern crate proc_macro;
extern crate syn;

use quote::quote;
use std::path::PathBuf;

use proc_macro::TokenStream;
use regex::Regex;
// use syn::{parse_macro_input, Attribute, DeriveInput, Error, Lit, Meta, MetaNameValue};
// let syn_input = parse_macro_input!(input as DeriveInput);
// let root_path = syn_input
//     .attrs
//     .iter()
//     .filter_map(|attr| match get_root_path(attr) {
//         Ok(v) => v,
//         Err(_) => panic!("Failed to parse attribute root_path; eg #[root_path = \"...\"]"),
//     })
//     .next()
//     .expect(
//         "Must provide directory root_path relative to Cargo.toml; eg #[root_path = \"...\"]",
//     );

// fn get_root_path(attr: &Attribute) -> syn::Result<Option<String>> {

//     if !attr.path.is_ident("root_path") {
//         return Ok(None);
//     }

//     match attr.parse_meta()? {
//         Meta::NameValue(MetaNameValue {
//             lit: Lit::Str(lit_str),
//             ..
//         }) => Ok(Some(lit_str.value())),
//         _ => {
//             let message = "expected #[root_path = \"...\"]";
//             Err(Error::new_spanned(attr, message))
//         }
//     }
// }

fn find_solution_filenames(
    full_path: &PathBuf,
) -> Result<Vec<(usize, String, String)>, Box<dyn std::error::Error>> {
    let mut results: Vec<(usize, String, String)> = Vec::new();
    // TODO: eventually let this support year and such organization
    let expression = Regex::new("(day([0-9]+)).rs")?;
    for entry in std::fs::read_dir(full_path)? {
        let entry = entry?;
        let path = entry.path();
        let Some(filename) = path.file_name() else {
            continue;
        };
        let Some(strname) = filename.to_str() else {
            continue;
        };
        if expression.is_match(strname) {
            let caps = expression.captures(strname).unwrap();
            results.push((
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(1).unwrap().as_str().to_string(),
                caps.get(0).unwrap().as_str().to_string(),
            ));
        }
    }
    Ok(results)
}

#[proc_macro]
pub fn import_aoc_solutions(_input: TokenStream) -> TokenStream {
    // TODO: Support the optional override form, since the current mod import only supports src/
    let root_path = "src/";

    let solution_paths = {
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
        let full_path = std::path::Path::new(&root).join(root_path);
        if full_path.is_dir() {
            match find_solution_filenames(&full_path) {
                Ok(v) => v,
                Err(e) => panic!("Failed to find files in path {:?}: {:?}", full_path, e),
            }
        } else {
            panic!("root_path must be a path to a directory relative to Cargo.toml");
        }
    };

    let mods: proc_macro2::TokenStream = solution_paths
        .iter()
        .map(|(_, name, _)| format!("mod {};", name))
        .collect::<Vec<String>>()
        .join("\n")
        .parse()
        .unwrap();

    let arms: proc_macro2::TokenStream = solution_paths
        .iter()
        .map(|(num, name, _)| format!("AocDay::D(_, {}) => {}::Solve::new(day)?.run(),", num, name))
        .collect::<Vec<String>>()
        .join("\n")
        .parse()
        .unwrap();

    let expanded = quote! {
        #mods
        fn run(day: AocDay) -> Result<String, Box<dyn Error>> {
          match day {
            #arms
            _ => Err("Unknown day given".into()),
          }
        }
    };

    TokenStream::from(expanded)
}
