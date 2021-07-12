use actix_web::{web, get, Error, HttpResponse};
use http::StatusCode;
use crate::utils::read_quotes_json::{read_quotes_json};
use crate::utils::types::{QuotesList, Quote};
use serde::{Deserialize};

#[derive(Deserialize)]
struct AuthorEndpointParams {
    author: String,
}

#[get("/quotes/{author}")]
async fn get_quotes_by_author(
    info: web::Path<AuthorEndpointParams>,
) -> Result<HttpResponse, Error> {
    let quotes_list = read_quotes_json();
    let quotes_by_author_query_result: QuotesList = QuotesList {
        quotes: quotes_list
            .quotes
            .into_iter()
            .filter(|quote| {
                quote
                    .author
                    .to_ascii_lowercase()
                    .contains(&info.author.to_ascii_lowercase())
            })
            .collect::<Vec<Quote>>(),
    };

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(quotes_by_author_query_result))
}