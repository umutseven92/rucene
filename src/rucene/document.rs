use crate::rucene::token::Token;

pub struct Document {
    pub id: u32,
    pub tokens: Vec<Token>,
}

impl Document {
    pub fn new(id: u32, tokens: Vec<Token>) -> Self {
        Document { id, tokens }
    }
}

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
