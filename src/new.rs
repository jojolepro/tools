use std::fs::{create_dir_all, File,OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ops::Add;

use error::{ErrorKind, Result, ResultExt};
use templates::get_template;

/// Options for the New subcommand. If `version` is None, then it uses
/// the latest version available
#[derive(Clone, Debug)]
pub struct New {
    pub project_name: String,
    pub version: String,
    pub git: bool,
    pub repo: String,
    pub branch: String,
}

impl New {

    pub fn new(game_name: Option<String>, version: Option<String>) -> Self{
        let mut n = New::default();
        if let Some(name) = game_name{
            n.project_name = name;
        }

        if let Some(v) = version{
            n.version = v;
        }

        n
    }

    pub fn from_git(game_name: Option<String>, version: Option<String>,repo: String, branch: String) -> Self{
        let mut n = New::new(game_name,version);
        n.git = true;
        n.repo = repo;
        n.branch = branch;

        n
    }

    pub fn execute(&self) -> Result<()> {
        self.execute_inner()
            .chain_err(|| ErrorKind::New(self.project_name.clone()))
    }

    fn execute_inner(&self) -> Result<()> {
        let path = Path::new(&self.project_name);
        if path.exists() {
            bail!("Project directory {:?} already exists", path);
        }

        // Create a cargo project
        Command::new("cargo").arg("new").arg("--bin").arg(&self.project_name).output().expect("Failed to create project using cargo");

        // Get the Cargo.toml file
        let cargo_path: PathBuf = [&self.project_name,"Cargo.toml"].iter().collect();
        let mut cargo_file = OpenOptions::new().write(true).append(true).open(cargo_path).expect("Failed to open Cargo.toml");

        let amethyst_dep = if self.git{
            format!("amethyst = {{ git = \"{}\", branch = \"{}\" }}",self.repo,self.branch)
        }else{
            format!("amethyst = \"{}\"",self.version)
        };

        // Append the amethyst's version to the dependencies
        writeln!(cargo_file,"{}",amethyst_dep).chain_err(|| format!("Failed to write to Cargo.toml"))?;

        Ok(())
    }
}

impl Default for New {
    fn default() -> Self {
        New {
            project_name: "amethyst-game".to_owned(),
            version: String::from("*"),
            git: false,
            repo: String::from("https://github.com/amethyst/amethyst"),
            branch: String::from("develop"),
        }
    }
}
