use crate::plugins::{
    types::{Plugin, ProjktResult},
    utils::{fuzzy, FuzzyItemType},
};

pub struct ReadmeOptions {
    pub name: Option<String>,
}

pub struct Readme;

impl From<FuzzyItemType> for clap::PossibleValue<'_> {
    fn from(t: FuzzyItemType) -> Self {
        clap::PossibleValue::new({
            let box_a: Box<str> = Box::from(t.0.as_str());
            Box::leak(box_a)
        })
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
        fuzzy(Self::fetch(&opts)?, false)?;

        Ok(())
    }
}
