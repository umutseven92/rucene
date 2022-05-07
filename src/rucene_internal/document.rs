use crate::rucene_internal::token::Token;

#[derive(Debug)]
pub struct AnalysedDocument {
    pub id: u32,
    pub tokens: Vec<Token>,
}

impl AnalysedDocument {
    pub fn new(id: u32, tokens: Vec<Token>) -> Self {
        AnalysedDocument { id, tokens }
    }
}

impl PartialEq for AnalysedDocument {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.tokens == other.tokens
    }
}

impl Eq for AnalysedDocument {}

#[derive(Debug)]
pub struct DocumentResult {
    pub id: u32,
}

impl DocumentResult {
    pub fn from_id(id: u32) -> Self {
        DocumentResult { id }
    }

    pub fn from_ids(ids: Vec<u32>) -> Vec<Self> {
        ids.iter().map(|id| Self::from_id(*id)).collect()
    }
}

impl PartialEq for DocumentResult {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DocumentResult {}
