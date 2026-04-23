pub use chrono::Local;
pub use rand::distr::{Alphanumeric, SampleString};
pub use rand::rng;
pub use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Link {
    pub full_url: String,
    pub shortened_url: String,
    pub date: String,
}

pub fn generate_url() -> String {
    let mut shortened_url = "dt.url/".to_string();
    let mut rng = rng();
    let url_ending = Alphanumeric.sample_string(&mut rng, 8);
    // concatenate to dt.url/ string
    shortened_url.push_str(&url_ending);
    return shortened_url;
}
pub fn check_database(full_url: &String, shortened_url: &String) -> Result<bool> {
    let conn = Connection::open("urls_all.db")?;
    let query = format!(
        "SELECT full_url, shortened_url, date FROM urls_all WHERE full_url='{}' OR shortened_url='{}'",
        &full_url, &shortened_url
    );
    let mut stmt = conn.prepare(&query)?;
    let url_db_matches = stmt.query_map([], |row| {
        Ok(Link {
            full_url: row.get(0)?,
            shortened_url: row.get(1)?,
            date: row.get(2)?,
        })
    })?;

    if url_db_matches.count() == 0 {
        return Ok(true); // no match found, URL is unique
    }
    Ok(false) // match found, URL already exists
}

pub fn add_url_to_db(full_url: &String, shortened_url: String) {
    let now = Local::now();
    let today_date: String = now.date_naive().format("%y/%m/%d").to_string();
    let conn = Connection::open("urls_all.db").unwrap();
    conn.execute(
        "INSERT INTO URLS_ALL (full_url, shortened_url, date) VALUES (?1, ?2, ?3)",
        (full_url, shortened_url, today_date),
    )
    .unwrap();
}

pub fn show_all_urls() -> Result<()> {
    let conn = Connection::open("urls_all.db")?;
    let mut stmt = conn.prepare("SELECT * FROM urls_all")?;
    let mut url_db_matches = stmt.query([])?;
    while let Some(row) = url_db_matches.next()? {
        // Access columns by index
        let full_url: String = row.get(0)?;
        let shortened_url: String = row.get(1)?;
        let date: String = row.get(2)?;
        println!("Found: {}, {}, {}", full_url, shortened_url, date);
    }
    Ok(())
}

pub fn delete_entry(full_url: &String) -> Result<()> {
    let conn = Connection::open("urls_all.db")?;
    conn.execute("DELETE FROM urls_all WHERE full_url = ?1", [full_url])
        .unwrap();
    Ok(())
}
