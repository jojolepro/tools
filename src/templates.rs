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
) -> Result<(String,Vec<(&'static str,&'static str)>)> {
    // Get the correct template
    /*let template_name = templates()
        .iter()
        .filter(|n| **n == name)
            .max();*/

    let template_map = external::template_files();
    match template_map.get::<str>(&name) {
        Some(ref v) => Ok((name, (*v).clone())),
        None => Err(ErrorKind::UnknownTemplate(name).into()),
    }
}


// move somewhere else
pub fn get_amethyst_version() -> Result<String> {
    // checks in Cargo.toml and returns the amethyst version.
    // used by some amethyst-tool functions to detect if a code generation template can be applied to this amethyst version or not.
    unimplemented!()
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