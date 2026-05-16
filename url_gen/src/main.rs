use rand::thread_rng;
use rand::{Rng, distributions::Alphanumeric};
use regex::Regex;

fn main() {
    let test_url = "https://elpais.com/us/".to_string();

    let url_status = check_url_validity(&test_url);

    if url_status {
        let short_url = generate_url_ending();
        println!("{:?}", short_url)
    } else {
        println!("Invalid url: try again.")
    }
}

pub fn check_url_validity(url: &str) -> bool {
    let re = Regex::new(
        r"^(https?://|www\.)[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z]{2,})+(/[^\s]*)?$"
    ).unwrap();

    re.is_match(url)
}

pub fn generate_url_ending() -> String {
    //check the validity of the url with regex before generation
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
