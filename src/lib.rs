//! # Rucene
//!
//! A very simple, Lucene-like library for full-text search, for educational purposes.

use crate::rucene_internal::document::{AnalysedDocument, DocumentResult};
use crate::rucene_internal::inverted_index::InvertedIndex;
use crate::rucene_internal::query::Query;
use std::error::Error;

pub mod rucene_internal;

pub struct Rucene {
    inverted_index: InvertedIndex,
}

impl Rucene {
    pub fn new() -> Self {
        Rucene {
            inverted_index: InvertedIndex::new(),
        }
    }

    pub fn index(&mut self, document: AnalysedDocument) -> Result<(), Box<dyn Error>> {
        self.inverted_index.index(document)
    }

    pub fn search(self, query: Query) -> Result<Vec<DocumentResult>, Box<dyn Error>> {
        self.inverted_index.retrieve(query)
    }
}
