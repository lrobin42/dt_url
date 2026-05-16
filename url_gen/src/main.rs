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

/*
fn main() {
    let test_url = "https://elpais.com/us/".to_string();
    let short_url = generate_url();
    let url_ending_available = check_database(&test_url, &short_url).unwrap();
    if url_ending_available {
        add_url_to_db(&test_url, short_url.clone());
    }
    let test_short = "dt.url/Goq8R0o8".to_string();
    open_short_url(&test_short)

    //show_all_urls().unwrap();

    //open_url_in_browser(&test_url);
}

pub fn open_url_in_browser(url_string: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    Command::new("open").arg(url_string).spawn()?;

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "start", url_string])
        .spawn()?;

    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(url_string)
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("URL needed");

    Ok(())
}

pub fn open_short_url(short_url: &String) {
    let conn = Connection::open("urls_all.db").unwrap();
    let query = format!(
        "SELECT full_url, shortened_url, date FROM urls_all WHERE shortened_url='{}'",
        &short_url
    );
    let mut stmt = conn.prepare(&query).unwrap();
    let mut url_db_matches = stmt
        .query_map([], |row| {
            Ok(Link {
                full_url: row.get(0)?,
                shortened_url: row.get(1)?,
                date: row.get(2)?,
            })
        })
        .unwrap();

    let stored_url = url_db_matches
        .next()
        .expect("No URL found")
        .expect("Row mapping failed")
        .full_url;

    open_url_in_browser(&stored_url);
}
*/
