pub type ProjktResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait Plugin {
    type Opts;
    type Fetch;

    fn fetch(opts: &Self::Opts) -> ProjktResult<Self::Fetch>;

    fn exec(opts: Self::Opts) -> ProjktResult<()>;
}
