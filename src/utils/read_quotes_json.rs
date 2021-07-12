use crate::utils::types::QuotesList;
use std::fs::File;

pub fn read_quotes_json() -> QuotesList {
    let quotes_json = File::open("./quotes.json").expect("file should open read only");
    serde_json::from_reader(quotes_json).expect("JSON not serializable")
}
