use dt_url::*;
use std::process::Command;

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

//fix the query
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
