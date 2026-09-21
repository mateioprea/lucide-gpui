use std::fs;
use std::fs::File;
use std::io::Write;

fn upper_case_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    let first = chars.next().unwrap().to_ascii_uppercase().to_string();
    first + chars.as_str()
}

fn main() {
    let icon_paths = fs::read_dir("./lucide_icons").unwrap();
    let mut generated_source = String::new();
    let mut parsed_icons = icon_paths
        .map(|icon_path| {
            let entry = icon_path.unwrap();
            let entry_path = entry.path();
            let icon_path = entry_path.file_name().unwrap();
            let file_stem = entry_path.file_stem().unwrap();
            let new_file_name = file_stem
                .to_str()
                .unwrap()
                .split("-")
                .map(upper_case_first_letter)
                .collect::<Vec<_>>()
                .join("_");
            (new_file_name, icon_path.to_str().unwrap().to_string())
        })
        .collect::<Vec<_>>();

    parsed_icons.sort_by(|a, b| a.0.cmp(&b.0));
    generated_source.push_str("#[allow(non_camel_case_types)]\n");
    generated_source.push_str("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n");
    generated_source.push_str("pub enum LucideIcon {\n");

    for (new_file_name, _) in parsed_icons.clone() {
        generated_source.push_str(&format!("    {new_file_name},\n"));
    }
    generated_source.push_str("}\n");
    generated_source.push_str(
        "impl gpui_kit::component::IconNamed for LucideIcon {\n    fn path(self) -> gpui_kit::SharedString {\n           match self {\n",
    );
    for (new_file_name, icon_path) in parsed_icons.clone() {
        generated_source.push_str(&format!(
            "               LucideIcon::{new_file_name} => \"lucide_icons/{}\",\n",
            icon_path
        ));
    }
    generated_source.push_str("\n        }.into()\n    }\n}\n");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut file = File::create(format!("{}/lucide_icons.rs", out_dir)).unwrap();
    file.write_all(generated_source.as_bytes()).unwrap();
    println!("cargo:rerun-if-changed=icons");
}
