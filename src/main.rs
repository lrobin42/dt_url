use dt_url::*;
use iced::{
    Element, Length, Settings, Task, Theme,
    widget::{button, column, container, text, text_input},
};
use rand::thread_rng;
use rand::{Rng, distributions::Alphanumeric};
use regex::Regex;

use std::sync::OnceLock;

#[derive(Debug)]
pub enum UrlValidationError {
    RegexInitFailed(String),
}

impl std::fmt::Display for UrlValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlValidationError::RegexInitFailed(msg) => {
                write!(f, "Failed to initialize URL regex: {}", msg)
            }
        }
    }
}

impl std::error::Error for UrlValidationError {}

fn main() -> iced::Result {
    iced::application("URL Shortener", update, view)
        .theme(|_| Theme::Dark)
        .run_with(|| (UrlShortener::default(), Task::none()))
}

#[derive(Default)]
struct UrlShortener {
    input_url: String,
    short_url: String,
    error_message: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    UrlInputChanged(String),
    GenerateShortUrl,
}

fn update(state: &mut UrlShortener, message: Message) -> Task<Message> {
    match message {
        Message::UrlInputChanged(value) => {
            state.input_url = value;
            state.error_message = None; // clear error on new input
        }
        Message::GenerateShortUrl => {
            state.error_message = None;

            if state.input_url.is_empty() {
                state.error_message = Some("URL invalid, try again.".to_string());
                return Task::none();
            }

            match check_url_validity(&state.input_url) {
                Ok(true) => {
                    let generated = generate_url();
                    let available = check_database(&state.input_url, &generated).unwrap_or(false);
                    if available {
                        add_url_to_db(&state.input_url, generated.clone());
                        state.short_url = format!("dt.url/{}", generated);
                    } else {
                        state.error_message =
                            Some("Failed to generate unique short URL.".to_string());
                    }
                }
                Ok(false) => {
                    state.error_message = Some("URL invalid, try again.".to_string());
                }
                Err(e) => {
                    state.error_message = Some(format!("Validation error: {}", e));
                }
            }
        }
    }
    Task::none()
}

fn view(state: &UrlShortener) -> Element<Message> {
    let input = text_input("Enter URL...", &state.input_url)
        .on_input(Message::UrlInputChanged)
        .padding(10)
        .size(20);

    let generate_button = button("Generate Short URL")
        .padding(10)
        .on_press(Message::GenerateShortUrl);

    let error_display = match &state.error_message {
        Some(msg) => text(msg).color(iced::Color::from_rgb(0.85, 0.1, 0.1)),
        None => text(""),
    };

    let output = text(&state.short_url).size(24);

    let content = column![input, generate_button, error_display, output]
        .spacing(20)
        .padding(20)
        .width(Length::Fill);

    container(content)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

/* ---------------------------------------------------
   Existing Logic
----------------------------------------------------*/

use rusqlite::Connection;
use std::process::Command;

#[derive(Debug)]
struct Link {
    full_url: String,
    shortened_url: String,
    date: String,
}

fn check_url_validity(url: &str) -> Result<bool, UrlValidationError> {
    static URL_REGEX: OnceLock<Result<Regex, regex::Error>> = OnceLock::new();

    let regex = URL_REGEX
        .get_or_init(|| {
            Regex::new(
                r"^(https?://|www\.)[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z]{2,})+(/[^\s]*)?$"
            )
        })
        .as_ref()
        .map_err(|e| UrlValidationError::RegexInitFailed(e.to_string()))?;

    Ok(regex.is_match(url))
}

// Example URL generator
fn generate_url() -> String {
    //check the validity of the url with regex before generation

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

// Example DB check
fn check_database(
    _full_url: &String,
    _short_url: &String,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Replace with real DB lookup
    Ok(true)
}

// Example DB insert
fn add_url_to_db(full_url: &String, short_url: String) {
    let conn = Connection::open("urls_all.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls_all (
            full_url TEXT NOT NULL,
            shortened_url TEXT NOT NULL,
            date TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    let current_date = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO urls_all (full_url, shortened_url, date)
         VALUES (?1, ?2, ?3)",
        (&full_url, &short_url, &current_date),
    )
    .unwrap();
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
        .spawn()?;

    Ok(())
}

pub fn open_short_url(short_url: &String) {
    let conn = Connection::open("urls_all.db").unwrap();

    let query = format!(
        "SELECT full_url, shortened_url, date
         FROM urls_all
         WHERE shortened_url='{}'",
        short_url
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

    open_url_in_browser(&stored_url).unwrap();
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
