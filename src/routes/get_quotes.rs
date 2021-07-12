use actix_web::{get, Error, HttpResponse};
use http::StatusCode;
use crate::utils::read_quotes_json::read_quotes_json;

#[get("/quotes")]
async fn get_quotes() -> Result<HttpResponse, Error> {
    let quotes_list = read_quotes_json();

    Ok(HttpResponse::Ok().status(StatusCode::OK).json(quotes_list))
}
