use rucene::rucene_internal::token::Token;

/// The final step is token filtering. Here the stream of tokens can be adjusted, either by adding or removing the tokens or by changing them.
pub(crate) trait TokenFilter
where
    Self: Send + Sync,
{
    /// The filtering is done in-place.
    fn filter(&self, tokens: &mut Vec<Token>);
}

pub(crate) struct LowerCaseTokenFilter {}

/// Lowercases each token.
impl TokenFilter for LowerCaseTokenFilter {
    fn filter(&self, tokens: &mut Vec<Token>) {
        *tokens = tokens
            .iter()
            .map(|token| Token::new(token.value.to_lowercase()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::token_filters::{LowerCaseTokenFilter, TokenFilter};
    use rucene::rucene_internal::token::Token;

    #[test]
    fn lowercase_token_filter() {
        let mut source = vec![
            Token::new("The".to_string()),
            Token::new("Brown’s".to_string()),
            Token::new("fiftieth".to_string()),
            Token::new("wedding".to_string()),
            Token::new("anniversary".to_string()),
            Token::new("at".to_string()),
            Token::new("Cafe".to_string()),
            Token::new("Ole".to_string()),
        ];

        let expected = vec![
            Token::new("the".to_string()),
            Token::new("brown’s".to_string()),
            Token::new("fiftieth".to_string()),
            Token::new("wedding".to_string()),
            Token::new("anniversary".to_string()),
            Token::new("at".to_string()),
            Token::new("cafe".to_string()),
            Token::new("ole".to_string()),
        ];

        LowerCaseTokenFilter {}.filter(&mut source);

        assert_eq!(source, expected);
    }
}
