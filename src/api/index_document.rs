use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct IndexDocument {
    pub id: u32,
    pub body: String,
}

impl IndexDocument {
    #[cfg(test)]
    pub(crate) fn new(id: u32, body: String) -> Self {
        IndexDocument { id, body }
    }
}
