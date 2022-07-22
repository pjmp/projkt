use std::{fs::OpenOptions, path::PathBuf};

use crate::plugins::{
    types::{Plugin, ProjktResult},
    utils::{fuzzy, write_or_create, FuzzyItemType},
};

pub struct ReadmeOptions {
    pub name: Option<String>,
    pub overwrite: bool,
}

pub struct Readme;

impl From<FuzzyItemType> for clap::PossibleValue<'_> {
    fn from(t: FuzzyItemType) -> Self {
        clap::PossibleValue::new(Box::leak(Box::from(t.0.as_str())))
    }
}

pub fn templates() -> Vec<FuzzyItemType> {
    vec![
        FuzzyItemType(
            "makeareadme".into(),
            include_str!("templates/makeareadme.md").into(),
        ),
        FuzzyItemType(
            "othneildrew".into(),
            include_str!("templates/othneildrew.md").into(),
        ),
        FuzzyItemType(
            "readmeso".into(),
            include_str!("templates/readmeso.md").into(),
        ),
        FuzzyItemType(
            "embarkstudios".into(),
            include_str!("templates/embarkstudios.md").into(),
        ),
        FuzzyItemType("simple".into(), include_str!("templates/simple.md").into()),
    ]
}

impl Plugin for Readme {
    type Opts = ReadmeOptions;
    type Fetch = Vec<FuzzyItemType>;

    fn fetch(_: &Self::Opts) -> ProjktResult<Self::Fetch> {
        Ok(templates())
    }

    fn exec(opts: Self::Opts) -> ProjktResult<()> {
        let templates = Self::fetch(&opts)?;

        let Self::Opts {
            ref name,
            overwrite,
        } = opts;

        let write = |data| -> ProjktResult<()> {
            write_or_create(
                OpenOptions::new().write(true).create(true),
                PathBuf::from("README.md"),
                data,
                overwrite,
            )?;

            Ok(())
        };

        if let Some(ref name) = name {
            let item = templates.iter().find(|item| item.0 == *name).unwrap();

            write(item.1.as_bytes())?;
        } else {
            let selection = fuzzy(templates, false)?;

            if !selection.is_empty() {
                let item = &selection[0];

                write(item.output().as_bytes())?;
            }
        }

        Ok(())
    }
}
