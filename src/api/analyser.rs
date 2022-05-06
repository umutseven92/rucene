use crate::api::character_filters::CharacterFilter;
use crate::api::token_filters::TokenFilter;
use crate::api::tokenizers::Tokenizer;
use crate::IndexDocument;

pub(crate) struct Analyser {
    character_filters: Vec<Box<dyn CharacterFilter>>,
    tokenizer: Box<dyn Tokenizer>,
    token_filters: Vec<Box<dyn TokenFilter>>,
}

impl Analyser {
    pub fn new(
        character_filters: Vec<Box<dyn CharacterFilter>>,
        tokenizer: Box<dyn Tokenizer>,
        token_filters: Vec<Box<dyn TokenFilter>>,
    ) -> Self {
        Self {
            character_filters,
            tokenizer,
            token_filters,
        }
    }
    /// Analysis is composed of three steps: character filtering, tokenization, and token filtering.
    pub fn analyse(document: IndexDocument) {}
}
