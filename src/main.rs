extern crate serde;
extern crate serde_json;
use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use http::StatusCode;
use rand::seq::SliceRandom;
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

#[get("/")]
async fn get_random_quote() -> Result<HttpResponse, Error> {
    let quotes_list = read_quotes_file();
    let random_quote = quotes_list.quotes.choose(&mut rand::thread_rng());
    if let Some(quote) = random_quote {
        Ok(HttpResponse::Ok().status(StatusCode::OK).json(quote))
    } else {
        Ok(HttpResponse::Ok()
            .status(StatusCode::OK)
            .json("Something went wrong"))
    }
}

#[get("/quotes/{author}")]
async fn get_quotes_by_author(
    info: web::Path<AuthorEndpointParams>,
) -> Result<HttpResponse, Error> {
    let quotes_list = read_quotes_file();
    let quotes_by_author_query_result: QuotesList = QuotesList {
        quotes: quotes_list
            .quotes
            .into_iter()
            .filter(|quote|  quote.author.to_ascii_lowercase().contains(&info.author.to_ascii_lowercase()))
            .collect::<Vec<Quote>>(),
    };

    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json(quotes_by_author_query_result))
}

#[get("/quotes")]
async fn get_quotes() -> Result<HttpResponse, Error> {
    let quotes_list = read_quotes_file();

    Ok(HttpResponse::Ok().status(StatusCode::OK).json(quotes_list))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_random_quote)
            .service(get_quotes)
            .service(get_quotes_by_author)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
