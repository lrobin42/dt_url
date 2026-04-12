use dt_url::*;

fn main() {
    let test_url = "https://elpais.com/us/".to_string();
    let short_url = generate_url();
    let url_ending_available = check_database(&test_url, &short_url).unwrap();
    if url_ending_available {
        add_url_to_db(test_url, short_url);
    }
    show_all_urls().unwrap();
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
