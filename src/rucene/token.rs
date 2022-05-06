pub type Tokens = Vec<Token>;

#[derive(Debug)]
pub struct Token {
    pub value: String,
}

impl Token {
    pub fn new(value: String) -> Self {
        Token { value }
    }
}

impl PartialEq<Self> for Token {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Token {}
