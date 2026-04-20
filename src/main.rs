use dt_url::*;

fn main() {
    let test_url = "https://elpais.com/us/".to_string();
    let short_url = generate_url();
    let url_ending_available = check_database(&test_url, &short_url).unwrap();
    if url_ending_available {
        add_url_to_db(&test_url, short_url);
    }
    show_all_urls().unwrap();

    open_url_in_browser(&test_url);
}

// let news = Link {
//     full_url: "www.vox.com".to_string(),
//     shortened_url: "dt.url/qxnpxmmj".to_string(),
//     date: "26/04/11".to_string(),
// };

//Receive the long url in the request body

/*
1. Receive the long URL in the request body
2. Validate it's an actual URL (not empty, has a scheme like `https://`)
3. Generate a random short code — look into the `nanoid` or `rand` crate
4. Check Redis: if the code already exists, generate a new one (collision handling)
5. Store `code → url` in Redis, optionally with a TTL (expiry time)
6. Return the full short URL to the caller (e.g. `https://yourdomain.com/x7kQpZ`)
*/

/*
| `POST`   | `/shorten` | Accept a long URL, return a short one          |
| -------- | ---------- | ---------------------------------------------- |
| `GET`    | `/:code`   | Look up the code, redirect to the original URL |
| `DELETE` | `/:code`   | Optional — remove a mapping                    |
*/

use std::process::Command;

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
}
