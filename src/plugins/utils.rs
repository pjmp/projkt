use std::{borrow::Cow, sync::Arc, vec};

use skim::{
    prelude::{unbounded, SkimOptionsBuilder},
    CaseMatching, ItemPreview, PreviewContext, Skim, SkimItem, SkimItemReceiver, SkimItemSender,
};

use crate::plugins::types;

pub struct FuzzyItemType(pub String, pub String);

impl SkimItem for FuzzyItemType {
    fn text(&self) -> Cow<str> {
        Cow::Owned(self.0.to_string())
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(self.1.to_string())
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.1)
    }
}

pub fn fuzzy(it: Vec<FuzzyItemType>) -> types::ProjktResult<Vec<Arc<dyn SkimItem>>> {
    let skim_options = SkimOptionsBuilder::default()
        .case(CaseMatching::Smart)
        .height(Some("70%"))
        .multi(true)
        .no_mouse(false)
        .preview_window(Some("right:80%"))
        .preview(Some(""))
        .reverse(true)
        .sync(true)
        .build()?;

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for item in it {
        tx.send(Arc::new(item))?;
    }

    drop(tx);

    if let Some(out) = Skim::run_with(&skim_options, Some(rx)) {
        if out.is_abort {
            return Err("".into());
        }

        return Ok(out.selected_items);
    }

    Ok(vec![])
}

pub fn prompt(msg: String) -> types::ProjktResult<bool> {
    let ans = dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(msg)
        .interact()?;

    Ok(ans)
}
