use crate::models::MeaningId;
use langtag::LangTagBuf;
use std::collections::BTreeSet;
use typed_builder::TypedBuilder;

use super::WordId;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(mutators(
    fn with_meaning(&mut self, meaning_id: MeaningId) {
        self.meaning_ids.insert(meaning_id);
    }
))]
pub struct Word {
    #[builder(default = WordId::new())]
    pub id: WordId,
    pub content: String,
    #[builder(default, via_mutators)]
    pub meaning_ids: BTreeSet<MeaningId>,
    #[builder(default, setter(strip_option))]
    pub language: Option<LangTagBuf>,
}
