use actix_web::{get, App, Error, HttpResponse, HttpServer};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Quotes {
    author: String,
    quote: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct QuotesList {
    quotes: Vec<Quotes>,
}

fn _read_quotes_file() -> QuotesList {
    let mut file = File::open("./quotes.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    serde_json::from_str(&buff).expect("Parsing quotes")
}

fn read_quotes_file() -> serde_json::Value {
    let quotes_json = File::open("./quotes.json").expect("file should open read only");
    serde_json::from_reader(quotes_json).expect("JSON not serializable")
}

#[get("/quotes/{author}")]
async fn get_quotes_by_author(author: String) -> Result<HttpResponse, Error> {
    let quotes_json= _read_quotes_file();

    Ok(HttpResponse::Ok().status(StatusCode::OK).json(quotes_json.quotes))
}

#[get("/quotes")]
async fn get_quotes() -> Result<HttpResponse, Error> {
    let quotes = read_quotes_file();
    // println!("{:?}", quotes.as_object().unwrap().get("quotes").unwrap().as_array().unwrap());

    Ok(HttpResponse::Ok().status(StatusCode::OK).json(quotes))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_quotes).service(get_quotes_by_author))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
