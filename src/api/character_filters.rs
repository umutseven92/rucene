use regex::Regex;

/// During the first step, character filtering, the characters of text fields can be adjusted or filtered in various ways.
/// A good example is HTMLStripCharFilter, which takes HTML as input and returns only the text contained within the HTML and not the HTML tags.
pub(crate) trait CharacterFilter {
    fn filter(input: String) -> String
    where
        Self: Sized;
}
/// Removes HTML tags from the input.
pub(crate) struct HTMLCharacterFilter {}

impl CharacterFilter for HTMLCharacterFilter {
    fn filter(input: String) -> String {
        // I know, I know, you should not use regex to parse HTML.
        // In this case though, we just want to strip them out, so in this limited use case it
        // should be fine.
        // The alternative is to use a library like scraper, which is a great library but not worth
        // the additional dependency.
        let re = Regex::new(r"(<[^>]*>)").unwrap();

        let result = re.replace_all(input.as_str(), "");

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::character_filters::{CharacterFilter, HTMLCharacterFilter};

    #[test]
    fn html_char_filter() {
        // Example sentence taken from Relevant Search by Doug Turnbull and Jerry Berryman.
        let source = "<h1>The Brown’s fiftieth wedding anniversary, at Café Olé.</h1>";
        let expected = "The Brown’s fiftieth wedding anniversary, at Café Olé.";

        let filtered = HTMLCharacterFilter::filter(String::from(source));

        assert_eq!(filtered, expected);
    }
}
