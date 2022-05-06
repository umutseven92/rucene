use crate::rucene::token::{Token, Tokens};

/// Basically a copy of Lucene's `BooleanQuery`, with `must`, `must_not` and `should` clauses.
/// # Example
/// (taken from Relevant Search by Doug Turnbull and Jerry Berryman)
/// ```
///
/// let query = Query::new(
/// vec![
///     Token::new("cat".to_string()),
/// ],
/// vec![Token::new("dog".to_string())],
/// vec![Token::new("black".to_string())],
/// );
/// ```
/// Documents:
/// * my cat ran under the couch
/// * black cats are mysterious
/// * the dog scared the black cat
///
/// This query is looking for any documents that MUST contain cat, SHOULD contain black, and MUST_NOT contain dog.
/// Therefore, both document (a) and (b) are matches because they contain the required term cat.
/// Of these two documents, (b) will rank more highly than (a) because it contains the nonrequired term black.
/// Even though document (c) contains both black and cat, it isn’t considered a match because it contains the disallowed term dog.
pub struct Query {
    pub must: Vec<Token>,
    pub must_not: Vec<Token>,
    pub should: Vec<Token>,
}

impl Query {
    pub fn new(must_tokens: Tokens, must_not_tokens: Tokens, should_tokens: Tokens) -> Self {
        Query {
            must: must_tokens,
            must_not: must_not_tokens,
            should: should_tokens,
        }
    }
}
