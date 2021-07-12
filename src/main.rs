extern crate serde;
extern crate serde_json;
use actix_web::{web, get, App, Error, HttpResponse, HttpServer};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct QuotesList {
    quotes: Vec<Quote>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Quote {
    author: String,
    quote: String,
}

#[derive(Deserialize)]
struct AuthorEndpointParams {
    author: String,
}

fn read_quotes_file() -> QuotesList {
    let quotes_json = File::open("./quotes.json").expect("file should open read only");
    serde_json::from_reader(quotes_json).expect("JSON not serializable")
}

#[get("/quotes/{author}")]
async fn get_quotes_by_author(info: web::Path<AuthorEndpointParams>) -> Result<HttpResponse, Error> {
    let quotes_json = read_quotes_file();
    let quotes_by_author_query_result: QuotesList = QuotesList {
        quotes: quotes_json
            .quotes
            .into_iter()
            .filter(|quote| quote.author == info.author)
            .collect::<Vec<Quote>>(),
    };

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(quotes_by_author_query_result))
}

#[get("/quotes")]
async fn get_quotes() -> Result<HttpResponse, Error> {
    let quotes = read_quotes_file();

    Ok(HttpResponse::Ok().status(StatusCode::OK).json(quotes))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_quotes).service(get_quotes_by_author))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
