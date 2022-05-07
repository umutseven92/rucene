use regex::Regex;
use rucene::rucene_internal::token::Token;

/// The next step is tokenization. As the name indicates, during this step raw text is converted into a stream of tokens.
/// There can be only one tokenizer in any given analysis chain.
pub(crate) trait Tokenizer
where
    Self: Send + Sync,
{
    fn tokenize(&self, input: &String) -> Vec<Token>;
}

/// Splits on whitespace and punctuation.
pub(crate) struct StandardTokenizer {}

impl Tokenizer for StandardTokenizer {
    fn tokenize(&self, input: &String) -> Vec<Token> {
        let re = Regex::new(r"\s|\.|,|-").unwrap();
        let result = re.split(input.as_str());

        result
            .filter(|word| !word.is_empty())
            .map(|word| Token::new(word.to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::tokenizers::{StandardTokenizer, Tokenizer};
    use rucene::rucene_internal::token::Token;

    #[test]
    fn standard_tokenizer() {
        // Example sentence taken from Relevant Search by Doug Turnbull and Jerry Berryman.
        let source = "The Brown’s fiftieth wedding anniversary, at Cafe Ole.";
        let expected = vec![
            Token::new("The".to_string()),
            Token::new("Brown’s".to_string()),
            Token::new("fiftieth".to_string()),
            Token::new("wedding".to_string()),
            Token::new("anniversary".to_string()),
            Token::new("at".to_string()),
            Token::new("Cafe".to_string()),
            Token::new("Ole".to_string()),
        ];

        let result = StandardTokenizer {}.tokenize(&source.to_string());

        assert_eq!(result, expected);
    }
}
