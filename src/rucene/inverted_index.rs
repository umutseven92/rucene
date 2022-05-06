use crate::rucene::document::{Document, DocumentResult};
use crate::rucene::query::Query;
use crate::rucene::token::Tokens;
use crate::rucene::utils::filter_vector;
use std::collections::BTreeMap;
use std::error::Error;

// These are BTree's rather than HashMap's, since HashMaps are not ordered.
type TermDictionary = BTreeMap<String, u32>;
type PostingsList = BTreeMap<u32, Vec<u32>>;

/// `InvertedIndex` is the heart of Rucene.
pub(crate) struct InvertedIndex {
    id_acc: u32,

    // Maps terms to ordinal numbers that uniquely identify a term.
    term_dictionary: TermDictionary,

    // Maps term ID to ID's of documents that contain the term.
    postings_list: PostingsList,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            id_acc: 0,
            term_dictionary: TermDictionary::new(),
            postings_list: PostingsList::new(),
        }
    }

    /// Index the document in the `InvertedIndex`.
    pub(crate) fn index(&mut self, document: Document) -> Result<(), Box<dyn Error>> {
        for token in document.tokens {
            match self.term_dictionary.get(&token.value) {
                Some(val) => {
                    // Term exists in the dictionary; a document containing the term has been indexed before.
                    self.postings_list
                        .get_mut(val)
                        .ok_or_else(|| {
                            format!(
                                // If a term exist in the term dictionary, then it should exist in
                                // the posting list as well, since we add it to the posting list when created.
                                "Value {0} is in the term dictionary but not in the posting list.",
                                val
                            )
                        })?
                        .push(document.id);
                }
                None => {
                    // Term does not exist in the dictionary; a document containing the term has not
                    // been indexed before.
                    self.term_dictionary
                        .insert(token.value.clone(), self.id_acc);

                    self.postings_list.insert(self.id_acc, vec![document.id]);
                    self.id_acc += 1;
                }
            };
        }

        Ok(())
    }

    /// Retrieve the list of document in the `InvertedIndex`, based on a `Query`.
    pub(crate) fn retrieve(self, query: Query) -> Result<Vec<DocumentResult>, Box<dyn Error>> {
        let must_results = self.get_ids_from_tokens(query.must)?;
        let must_not_results = self.get_ids_from_tokens(query.must_not)?;

        let filtered_results = filter_vector(must_results, must_not_results);

        Ok(DocumentResult::from_ids(filtered_results))
    }

    #[cfg(test)]
    fn get_documents_from_tokens(
        &self,
        tokens: Tokens,
    ) -> Result<Vec<DocumentResult>, Box<dyn Error>> {
        let doc_ids: Vec<u32> = self.get_ids_from_tokens(tokens)?;

        Ok(DocumentResult::from_ids(doc_ids))
    }

    fn get_ids_from_tokens(&self, tokens: Tokens) -> Result<Vec<u32>, Box<dyn Error>> {
        let mut doc_ids: Vec<u32> = vec![];

        for token in tokens {
            if let Some(id) = self.term_dictionary.get(token.value.as_str()) {
                let document_ids = self.postings_list.get(id).ok_or_else(|| {
                    format!(
                        // If a term exist in the term dictionary, then it should exist in
                        // the posting list as well, since we add it to the posting list when created.
                        "Value {0} is in the term dictionary but not in the posting list.",
                        id
                    )
                })?;

                // Remove duplicate document IDs.
                document_ids.iter().for_each(|id| {
                    if !doc_ids.contains(id) {
                        doc_ids.push(*id);
                    }
                });
            } else {
                // If the term does not exist in the dictionary, then it means that no document containing the term has been indexed.
                break;
            }
        }

        Ok(doc_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rucene::token::Token;

    fn init_test_inv_index() -> InvertedIndex {
        let mut inv_index = InvertedIndex::new();

        let documents = vec![
            Document::new(
                0,
                vec![
                    Token::new(String::from("back")),
                    Token::new(String::from("to")),
                    Token::new(String::from("the")),
                    Token::new(String::from("future")),
                ],
            ),
            Document::new(
                1,
                vec![
                    Token::new(String::from("future")),
                    Token::new(String::from("cop")),
                ],
            ),
            Document::new(
                2,
                vec![
                    Token::new(String::from("back")),
                    Token::new(String::from("again")),
                ],
            ),
        ];

        for doc in documents {
            let result = inv_index.index(doc);

            assert!(result.is_ok());
        }

        inv_index
    }

    #[test]
    fn can_index_term_dictionary() {
        let inv_index = init_test_inv_index();

        let expected_term_dict = BTreeMap::from([
            (String::from("again"), 5),
            (String::from("back"), 0),
            (String::from("cop"), 4),
            (String::from("future"), 3),
            (String::from("the"), 2),
            (String::from("to"), 1),
        ]);

        assert_eq!(expected_term_dict, inv_index.term_dictionary);
    }

    #[test]
    fn can_index_postings_list() {
        let inv_index = init_test_inv_index();

        let expected_posting_list = BTreeMap::from([
            (5, vec![2]),
            (0, vec![0, 2]),
            (4, vec![1]),
            (3, vec![0, 1]),
            (2, vec![0]),
            (1, vec![0]),
        ]);

        assert_eq!(expected_posting_list, inv_index.postings_list);
    }

    #[test]
    fn can_get_must_documents() {
        let inv_index = init_test_inv_index();

        let tokens = vec![Token::new("future".to_string())];

        let results = inv_index.get_documents_from_tokens(tokens).unwrap();

        assert_eq!(results.len(), 2);

        assert_eq!(
            results,
            vec![DocumentResult::from_id(0), DocumentResult::from_id(1)]
        );

        let tokens = vec![
            Token::new("back".to_string()),
            Token::new("again".to_string()),
        ];

        let results = inv_index.get_documents_from_tokens(tokens).unwrap();

        assert_eq!(results.len(), 2);

        assert_eq!(
            results,
            vec![DocumentResult::from_id(0), DocumentResult::from_id(2)]
        );

        let tokens = vec![Token::new("the".to_string())];

        let results = inv_index.get_documents_from_tokens(tokens).unwrap();

        assert_eq!(results.len(), 1);

        assert_eq!(results, vec![DocumentResult::from_id(0)]);
    }

    #[test]
    fn can_retrieve() {
        let inv_index = init_test_inv_index();

        let query = Query::new(
            vec![
                Token::new("back".to_string()),
                Token::new("future".to_string()),
            ],
            vec![Token::new("cop".to_string())],
            vec![],
        );

        let result = inv_index.retrieve(query).unwrap();

        assert_eq!(
            result,
            vec![DocumentResult::from_id(0), DocumentResult::from_id(2)]
        );
    }
}
