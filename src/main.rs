#[macro_use]
extern crate rocket;

use crate::api::analyser::Analyser;
use crate::api::index_document::IndexDocument;
use crate::api::responses::IndexResponse;
use ::rucene::Rucene;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Mutex;

mod api;

#[post("/index_doc", format = "json", data = "<document>")]
fn index_doc(
    document: Json<IndexDocument>,
    rucene: &State<Mutex<Rucene>>,
    analyser: &State<Analyser>,
) -> Result<Json<IndexResponse>, Status> {
    let analysed_doc = analyser.analyse(&document);

    match rucene.lock().unwrap().index(analysed_doc) {
        Ok(_) => Ok(Json(IndexResponse {
            success: true,
            error: String::new(),
        })),
        Err(err) => Ok(Json(IndexResponse {
            success: false,
            error: err.to_string(),
        })),
    }
}

#[get("/search")]
fn search() {
    todo!()
}

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    rocket::build()
        .manage(Mutex::new(Rucene::new()))
        .manage(Analyser::standard())
        .mount("/", routes![index_doc])
        .mount("/", routes![search])
        .launch()
        .await;
}
