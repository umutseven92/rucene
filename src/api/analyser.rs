use crate::api::character_filters::{CharacterFilter, HTMLCharacterFilter};
use crate::api::token_filters::{LowerCaseTokenFilter, TokenFilter};
use crate::api::tokenizers::{StandardTokenizer, Tokenizer};
use crate::IndexDocument;
use rucene::rucene_internal::document::AnalysedDocument;
use rucene::rucene_internal::token::Token;

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

    pub fn standard() -> Self {
        Self::new(
            vec![Box::new(HTMLCharacterFilter {})],
            Box::new(StandardTokenizer {}),
            vec![Box::new(LowerCaseTokenFilter {})],
        )
    }

    /// Analysis is composed of three steps: character filtering, tokenization, and token filtering.
    pub fn analyse(&self, document: &IndexDocument) -> AnalysedDocument {
        let mut body = document.body.clone();

        self.run_character_filters(&mut body);
        let mut tokens = self.run_tokenizer(&body);
        self.run_token_filters(&mut tokens);

        AnalysedDocument::new(document.id, tokens)
    }

    fn run_character_filters(&self, input: &mut String) {
        for filter in &self.character_filters {
            (*filter).filter(input);
        }
    }

    fn run_tokenizer(&self, input: &String) -> Vec<Token> {
        (*self.tokenizer).tokenize(input)
    }

    fn run_token_filters(&self, input: &mut Vec<Token>) {
        for filter in &self.token_filters {
            (*filter).filter(input);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::analyser::Analyser;
    use crate::api::character_filters::HTMLCharacterFilter;
    use crate::api::token_filters::LowerCaseTokenFilter;
    use crate::api::tokenizers::StandardTokenizer;
    use crate::IndexDocument;
    use rucene::rucene_internal::document::AnalysedDocument;
    use rucene::rucene_internal::token::Token;

    fn init_analyser() -> Analyser {
        Analyser::new(
            vec![Box::new(HTMLCharacterFilter {})],
            Box::new(StandardTokenizer {}),
            vec![Box::new(LowerCaseTokenFilter {})],
        )
    }

    #[test]
    fn can_analyse() {
        let analyser = init_analyser();
        let document = IndexDocument::new(
            0,
            String::from("<h1>The Brown’s fiftieth wedding anniversary, at Café Olé.</h1>"),
        );

        let expected_doc = AnalysedDocument::new(
            0,
            vec![
                Token::new("the".to_string()),
                Token::new("brown’s".to_string()),
                Token::new("fiftieth".to_string()),
                Token::new("wedding".to_string()),
                Token::new("anniversary".to_string()),
                Token::new("at".to_string()),
                Token::new("café".to_string()),
                Token::new("olé".to_string()),
            ],
        );

        let result = analyser.analyse(&document);

        assert_eq!(result, expected_doc);
    }
}
