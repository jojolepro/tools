/// Fetches the latest version of Amethyst by pulling from crates.io
/// Most of this code is based off of cargo-edit's fetch code
use std::fs;

use semver;

use error::*;

mod external {
    include!(concat!(env!("OUT_DIR"), "/_template_files.rs"));
}

pub fn get_template(
    name: String,
) -> Result<String> {
    // Get the correct template
    let version = templates()
        .filter(|n| n.matches(name))
        .max()
    let ver_str = format!("v{}.{}", version.major, version.minor);
    eprintln!("Using template for version {:?}", ver_str);
    let template_map = external::template_files();
    match template_map.get::<str>(&ver_str) {
        Some(ref v) => Ok((ver_str.get(1..).unwrap().to_owned(), (*v).clone())),
        None => Err(ErrorKind::UnsupportedVersion(ver_str).into()),
    }
}


// move somewhere else
pub fn get_amethyst_version() -> Result<String,()> {
    // checks in Cargo.toml and returns the amethyst version.
    // used by some amethyst-tool functions to detect if a code generation template can be applied to this amethyst version or not.
}


pub fn templates() -> Vec<String>{
    fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/project-templates"))
        .expect("Failed to find templates folder")
        .map(|dir| {
            let file = dir.expect("Failed to read template folder information");
            file.file_name().into_string().unwrap()
        }).collect::<Vec<String>>()
}

pub fn list_templates(){
    println!("Available Templates");
    for t in templates(){
        println!("{}",t);
    }
}