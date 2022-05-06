use crate::rucene::token::Token;

/// The final step is token filtering. Here the stream of tokens can be adjusted, either by adding or removing the tokens or by changing them.
pub(crate) trait TokenFilter {
    fn filter(tokens: &Vec<Token>) -> Vec<Token>
    where
        Self: Sized;
}

pub(crate) struct LowerCaseTokenFilter {}

/// Lowercases each token.
impl TokenFilter for LowerCaseTokenFilter {
    fn filter(tokens: &Vec<Token>) -> Vec<Token> {
        tokens
            .iter()
            .map(|token| Token::new(token.value.to_lowercase()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::token_filters::{LowerCaseTokenFilter, TokenFilter};
    use crate::rucene::token::Token;

    #[test]
    fn lowercase_token_filter() {
        let source = vec![
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

        let result = LowerCaseTokenFilter::filter(&source);

        assert_eq!(result, expected);
    }
}
