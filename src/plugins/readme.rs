use crate::plugins::types::{Plugin, ProjktResult};

pub struct ReadmeOptions;

pub struct Readme;

impl Plugin for Readme {
    type Opts = ReadmeOptions;
    type Fetch = ();

    fn fetch(_: &Self::Opts) -> ProjktResult<Self::Fetch> {
        unimplemented!()
    }

    fn exec(_: Self::Opts) -> ProjktResult<()> {
        unimplemented!()
    }
}
