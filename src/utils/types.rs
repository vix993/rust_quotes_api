extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuotesList {
    pub quotes: Vec<Quote>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Quote {
    pub author: String,
    pub quote: String,
}