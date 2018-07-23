//! Scraper
//!
//! Library for scanning various eSports websites and databases to keep a constant
//! list of tournaments. Attempts to normalize and categorize them as well.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/23/2017

extern crate rand;
extern crate serde;
extern crate chrono;
extern crate dotenv;
extern crate reqwest;
extern crate sentry;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

use std::env;
use std::process;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use sentry::integrations::panic::register_panic_handler;

mod schema;
mod models;
mod smashgg;
mod http;
mod util;

fn log_and_die(ty: &str, msg: String) {
    println!("{}", msg);
    sentry::capture_exception(&ty, Some(msg));
    process::exit(1);
}

fn main() {
    dotenv().ok();
    
    let dsn = env::var("SENTRY_DSN").expect("SENTRY_DSN not set?");
    let _sentry = sentry::init(dsn);
    register_panic_handler();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set?");
    let db = PgConnection::establish(&db_url).expect("Error connecting to database!");
    let client = http::configure_client().expect("Error configuring HTTP Client!");
    
    if let Err(err) = smashgg::fetch_and_store_tournaments(client, db) {
        let fmtd = format!("Error fetching tournaments from Smash.GG: {}", err);
        log_and_die("SmashGG", fmtd);
    }
}
