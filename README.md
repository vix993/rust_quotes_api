# rust_quotes_api

An api that will give you iconic quotes by famous authors and public figures.

It was built using Actix-web and Rust.

## Instructions

You can run the api by having rust and cargo [installed](https://doc.rust-lang.org/book/ch01-01-installation.html).

Then you can run `cargo build`, `cargo check`, `cargo test` and `cargo run` in the root directory.

### /
#### Get a random quote

Request -> `curl 127.0.0.1:8080/`
<br>
<br>
Response ->
```{
      "author": "Confucius",
      "quote": "Everything has beauty, but not everyone can see."
   }```


### /quotes
#### Get all quotes

Request -> `curl 127.0.0.1:8080/quotes/`
<br>
<br>
Response -> `{ "quotes": [ { "author": "Confucius", "quote":"Everything has beauty, but not everyone can see." }, { "author": "Farrah Gray", "quote":"Build your own dreams, or someone else will hire you to build theirs." }, ... ] }`

### /quotes/{author}
#### Get a list of quotes by the author or filter by string

Request -> `curl 127.0.0.1:8080/quotes/Albert Einstein` or `curl 127.0.0.1:8080/quotes/Albert`
<br>
<br>
Response -> `{ "quotes": [ { "author": "Albert Einstein", "quote": "Strive not to be a success, but rather to be of value." }, { "author": " Albert Einstein", "quote": "A person who never made a mistake never tried anything new." } ] }`
