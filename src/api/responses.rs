use rocket::serde::Serialize;

#[derive(Serialize)]
pub(crate) struct IndexResponse {
    pub success: bool,
    pub error: String,
}
