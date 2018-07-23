//! util.rs
//!
//! Utility methods and such. Fairly self explanatory by method name.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/29/2018

use rand;
use rand::Rng;
use std::{thread, time};

/**
 *  This is a necessary function for pretty much each scraper, as... well,
 *  some APIs don't sanitize null values in response strings, and postgres
 *  ain't about that life.
 */
pub fn sanitize_optional_string(s: Option<String>) -> String {
    let x = s.unwrap_or("".into());
    if x == "" { return x; }

    return x.replace("\0", "");    
}

/**
 *  Any good scraper should sleep with random intervals, so it doesn't look like
 *  you're just abusing an endpoint or something. More sophisticated platforms
 *  won't fall for this, but eh, it's good practice.
 */
pub fn sleep_for_random_seconds() {
    let seconds = rand::thread_rng().gen_range(4, 10);
    let duration = time::Duration::new(seconds, 0);
    println!("    Sleeping for {} seconds...", seconds);
    thread::sleep(duration);
}
