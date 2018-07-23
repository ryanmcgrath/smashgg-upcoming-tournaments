//! http.rs
//!
//! Boilerplate for creating the HTTP client used for requesting
//! and scraping. I prefer to have it cycle user agents, and that looks
//! kind of verbose and annoying, so... it's over here.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/29/2018

use rand;
use rand::Rng;
use reqwest::{Client, Error};
use reqwest::header::{Headers, UserAgent};

/**
 *  Builds an HTTP client for scraping functions to use, with a random-ish user-agent
 *  string. Theoretically this could be pulled out into a config file or something, but
 *  this is fine for now.
 */
pub fn configure_client() -> Result<Client, Error> {
    let user_agents = [
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_1) AppleWebKit/601.2.7 (KHTML, like Gecko) Version/9.0.1 Safari/601.2.7",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36",
        "Mozilla/5.0 (compatible, MSIE 11, Windows NT 6.3; Trident/7.0; rv:11.0) like Gecko",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.139 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_4) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/11.2 Safari/605.1.15",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/61.0.3163.100 Safari/537.36"
    ];

    let ua = rand::thread_rng().choose(&user_agents).unwrap();
    let mut headers = Headers::new();
    headers.set(UserAgent::new(ua.to_string()));
    Ok(Client::builder().default_headers(headers).build()?)
}
