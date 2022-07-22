use std::path::PathBuf;

use clap::{AppSettings, Parser, Subcommand};

use crate::plugins::{
    gitignore, license, readme,
    types::{Plugin, ProjktResult},
};

#[derive(Debug, Parser)]
#[clap(version, about)]
#[clap(global_settings = &[AppSettings::DisableHelpSubcommand, AppSettings::DeriveDisplayOrder])]
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Overwrite file if it exist
    #[clap(short, long, value_parser)]
    overwrite: bool,

    /// Path to save output to
    #[clap(short, long, value_parser = validate_path, default_value = ".")]
    dest: PathBuf,
}

fn validate_path(path: &str) -> Result<PathBuf, String> {
    if PathBuf::from(path).is_dir() {
        Ok(path.into())
    } else {
        Err("No such directory".into())
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate gitignore file
    Gitignore {
        /// Gitignore template name, eg: rust, ocaml
        #[clap(value_parser)]
        name: Option<String>,

        /// Append template to existing .gitignore file
        #[clap(value_parser, short, long)]
        append: bool,
    },

    /// Generate license file(s)
    License {
        /// Name of author to copyright to, if not passed will try to get
        /// from git config, $PROJKT_AUTHOR, $USER in that order
        #[clap(short, long, value_parser)]
        author: Option<String>,

        /// Author's email, if not passed will try to get
        /// from git config, $PROJKT_EMAIL in that order
        #[clap(short, long, value_parser)]
        email: Option<String>,

        /// SPDX license identifier, eg:  Apache-2.0, MIT
        #[clap(value_parser, possible_values = license::License::get())]
        names: Vec<String>,
    },

    /// Generate readme file
    Readme {
        #[clap(value_parser, possible_values = readme::templates())]
        name: Option<String>,
    },
}

impl Cli {
    pub(crate) fn run() -> ProjktResult<()> {
        let Self {
            command,
            dest,
            overwrite,
        } = Self::parse();

        match command {
            Commands::Gitignore { name, append } => {
                let opts = gitignore::GitIgnoreOptions {
                    name,
                    dest,
                    overwrite,
                    append,
                };

                if append && overwrite {
                    return Err("--append & --overwrite doesn't make sense".into());
                }

                gitignore::GitIgnore::exec(opts)?;
            }

            Commands::License {
                names,
                author,
                email,
            } => {
                let opts = license::LicenseOptions {
                    author,
                    email,
                    overwrite,
                    names,
                };

                license::License::exec(opts)?;
            }

            Commands::Readme { name } => {
                readme::Readme::exec(readme::ReadmeOptions { name, overwrite })?;
            }
        }

        Ok(())
    }
}
