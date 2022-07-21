use std::{fs, io::Write, path::Path};

use crate::plugins::{
    types::{Plugin, ProjktResult},
    utils::{self, FuzzyItemType},
};

// Taken from: https://doc.rust-lang.org/stable/std/sync/struct.Once.html
mod cache {
    use std::sync::Once;

    type Value = &'static [(&'static str, &'static str)];

    static mut VAL: Value = &[];

    static LICENSE: Once = Once::new();

    pub fn get() -> Value {
        unsafe {
            LICENSE.call_once(|| {
                VAL = spdx::text::LICENSE_TEXTS;
            });

            VAL
        }
    }
}

pub struct LicenseOptions {
    pub author: Option<String>,
    pub email: Option<String>,
    pub overwrite: bool,
    pub names: Vec<String>,
}

pub struct License;

impl License {
    pub fn get() -> Vec<&'static str> {
        cache::get()
            .iter()
            .map(|(id, _)| *id)
            .collect::<Vec<&str>>()
    }

    fn write(data: Vec<(String, String)>, overwrite: bool) -> ProjktResult<()> {
        let changed = data
            .iter()
            .try_fold(false, |mut state, item| -> ProjktResult<bool> {
                let file_name = format!("LICENSE-{}", item.0);

                let overwrite = if Path::new(&file_name).exists() {
                    if overwrite {
                        true
                    } else {
                        utils::prompt(format!("overwrite '{file_name}'"))?
                    }
                } else {
                    true
                };

                if overwrite {
                    let mut file = fs::File::create(file_name)?;

                    file.write_all(item.1.as_bytes())?;

                    file.flush()?;

                    state = true;
                }

                Ok(state)
            })?;

        if changed {
            println!(
                r#"Note: license file(s) were created/modified. Please check manually to make sure
nothing is out of place such as year, author, email etc are filled"#
            )
        }
        Ok(())
    }
}

impl Plugin for License {
    type Opts = LicenseOptions;
    type Fetch = ();

    fn fetch(_: &Self::Opts) -> ProjktResult<Self::Fetch> {
        unimplemented!()
    }

    fn exec(opts: Self::Opts) -> ProjktResult<()> {
        let LicenseOptions {
            names, overwrite, ..
        } = opts;

        if names.is_empty() {
            let choices = cache::get()
                .iter()
                .map(|item| FuzzyItemType(item.0.into(), item.1.into()))
                .collect();

            let selection = utils::fuzzy(choices)?;

            let data = selection
                .iter()
                .map(|item| (item.text().to_string(), item.output().to_string()))
                .collect();

            Self::write(data, overwrite)?;
        } else {
            let data = names
                .iter()
                .map(|item| {
                    // can unwrap because clap validates by `possible_values` attribute
                    let it = cache::get().iter().find(|(x, _)| x == item).unwrap();

                    (it.0.to_owned(), it.1.to_owned())
                })
                .collect();

            Self::write(data, overwrite)?;
        }

        Ok(())
    }
}
