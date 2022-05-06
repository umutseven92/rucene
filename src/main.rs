#[macro_use]
extern crate rocket;

use crate::api::index_document::IndexDocument;
use rocket::serde::json::Json;

mod api;
mod rucene;

#[post("/index_doc", data = "<document>")]
fn index_doc(document: Json<IndexDocument>) -> String {
    format!("Document body is {0}", document.body)
}

#[get("/search")]
fn search() {
    todo!()
}

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    rocket::build()
        .mount("/", routes![index_doc])
        .mount("/", routes![search])
        .launch()
        .await;
}
