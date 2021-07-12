use actix_web::{get, Error, HttpResponse};
use http::StatusCode;
use rand::seq::SliceRandom;
use crate::utils::read_quotes_json::{read_quotes_json};

#[get("/")]
async fn get_random_quote() -> Result<HttpResponse, Error> {
    let quotes_list = read_quotes_json();
    let random_quote = quotes_list.quotes.choose(&mut rand::thread_rng());
    if let Some(quote) = random_quote {
        Ok(HttpResponse::Ok().status(StatusCode::OK).json(quote))
    } else {
        Ok(HttpResponse::Ok()
            .status(StatusCode::OK)
            .json("Something went wrong"))
    }
}