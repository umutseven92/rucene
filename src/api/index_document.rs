use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct IndexDocument {
    pub body: String,
}
