extern crate serde;
extern crate serde_json;
use rust_quotes_api::routes::{get_quotes, get_random_quote, get_quotes_by_author};
use actix_web::{App, HttpServer};

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
