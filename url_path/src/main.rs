use rand::distr::{Alphanumeric, SampleString};
use rand::rng;
use rusqlite::{Connection, Result};

fn main() {
    //let conn = Connection::open("urls_all.db")?;

    let test_url = "https://www.wsj.com/".to_string();
    let short = shorten_url(test_url);
    println!("{}", short)
}

fn shorten_url(full_url: String) -> String {
    let mut shortened_url = "dt.url/".to_string();
    //randomly generate 8 character alphanumeric string  dt.url/qxnpxmmj
    let mut rng = rng();
    let url_ending = Alphanumeric.sample_string(&mut rng, 8);
    // concatenate to dt.url/ string
    shortened_url.push_str(&url_ending);
    // check if the string is already in db
    // return string
    return shortened_url;
}
