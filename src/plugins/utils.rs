use std::{borrow::Cow, fs::OpenOptions, io::Write, path::PathBuf, sync::Arc};

use skim::{
    prelude::{unbounded, SkimOptionsBuilder},
    CaseMatching, ItemPreview, PreviewContext, Skim, SkimItem, SkimItemReceiver, SkimItemSender,
};

use crate::plugins::types::ProjktResult;

pub struct FuzzyItemType(pub String, pub String);

impl SkimItem for FuzzyItemType {
    fn text(&self) -> Cow<str> {
        Cow::Owned(self.0.to_string())
    }

    fn preview(&self, _: PreviewContext) -> ItemPreview {
        ItemPreview::Text(self.1.to_string())
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.1)
    }
}

pub fn fuzzy(it: Vec<FuzzyItemType>, multi: bool) -> ProjktResult<Vec<Arc<dyn SkimItem>>> {
    let skim_options = SkimOptionsBuilder::default()
        .case(CaseMatching::Smart)
        .multi(multi)
        .preview_window(Some("right:80%"))
        .preview(Some(""))
        .reverse(true)
        .header(Some("Select any available templates"))
        .build()?;

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for item in it {
        tx.send(Arc::new(item))?;
    }

    drop(tx);

    if let Some(out) = Skim::run_with(&skim_options, Some(rx)) {
        if out.is_abort {
            return Err("Aborted by user".into());
        }

        return Ok(out.selected_items);
    }

    Ok(vec![])
}

pub fn prompt(msg: String) -> ProjktResult<bool> {
    use dialoguer::{theme::ColorfulTheme, Confirm};

    let ans = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .interact()?;

    Ok(ans)
}

pub fn write_or_create(
    file_opts: &mut OpenOptions,
    path: &PathBuf,
    contents: &[u8],
    overwrite: bool,
) -> ProjktResult<bool> {
    let state = if path.exists() { "modify" } else { "create" };

    let display = if path.exists() {
        path.canonicalize()?
    } else {
        path.clone()
    };

    if !overwrite && !prompt(format!("{state} {display:?}"))? {
        return Ok(false);
    }

    let mut file = file_opts.open(&path)?;

    file.write_all(contents)?;

    file.flush()?;

    Ok(true)
}
