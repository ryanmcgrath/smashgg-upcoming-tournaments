//! smashgg.rs
//!
//! Module for fetching, parsing, and storing SmashGG tournament
//! entries. Attempts to normalize where possible (i.e, all the structs below
//! are primarily used for decoding their API JSON response).
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/23/2017

use diesel;
use reqwest;

use chrono::{NaiveDateTime};
use reqwest::Client;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use util;
use models::Tournament;

#[derive(Debug, Deserialize)]
struct Response { total_count: usize, items: Items }

#[derive(Debug, Deserialize)]
struct Items { entities: Entities }

#[derive(Debug, Deserialize)]
struct Entities { event: Vec<Event>, tournament: Vec<SmashGGTournament> }

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Event {
    id: i32,
    tournamentId: i32,
    name: String,
    slug: String,
    videogameId: i32,
    isOnline: bool,
    isPlaceholder: bool
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct SmashGGTournament {
    id: i32,
    name: String,
    details: Option<String>,
    startAt: i64,
    endAt: i64,
    hashtag: Option<String>,
    slug: String,
    timezone: String,
    addrState: Option<String>,
    city: Option<String>,
    hasOnlineEvents: bool,
    published: bool
}

/**
 *  Builds and returns an API URL for Smash.GG's (internal?) API.
 *  Their URLs are a bit odd in structure, so this is abstracted out to
 *  make it a bit more clear... and to support pagination/etc if deemed
 *  necessary.
 *
 *  Note that this drops the returnMeta=true that they seem to call, but
 *  that's fine as we don't tend to use that information.
 */
fn api_url(page: i32) -> String {
    return [
        "https://smash.gg/api/-/gg_api./public/tournaments/schedule",
        "filter={\"upcoming\":true}",
        &["page=", &page.to_string()].join(""),
        "per_page=100",
        "reset=false",
        "schedule=true"
    ].join(";");
}

/**
 *  Locations on Smash.gg can be... well, weird. Sometimes info exists and sometimes it doesn't.
 *  Thus, we'll kludge something together. Yes, there is probably a better way of doing this, and
 *  no, I don't care to figure it out at the moment.
 */
fn build_location_str(tournament: &SmashGGTournament) -> String {   
    // Oh thank god I found a way to do this reference.
    let mut city: String = "".into();
    
    match &tournament.city {
        None => (),
        Some(c) => city.push_str(&c)
    }

    match &tournament.addrState {
        None => (),
        Some(addr_state) => {
            if addr_state != "" {
                if city != "" {
                    city.push_str(", ");
                }

                city.push_str(&addr_state);
            }
        }
    }
    
    if city == "" && tournament.hasOnlineEvents {
        return "Online".into();
    }
    
    return city;
}

/**
 *  Tournaments have a URL. This is abstracted because I've noticed oddities regarding
 *  putting together urls for Smash.gg pages in the past, and I expect this to require touchup
 *  at some point in the future.
 */
fn build_tournament_url(tournament: &SmashGGTournament) -> String {
    return ["https://smash.gg/", &tournament.slug].join("");
}

/**
 *  Possibly temporary depending on how other sites get scanned.
 */
fn collect_games(tid: i32, events: &Vec<Event>) -> String {
    let mut games: Vec<String> = Vec::new();

    for event in events {
        if event.tournamentId == tid {
            games.push(event.videogameId.to_string());
        }
    }

    return games.join(",");
}

/**
 *  Handles actually calling out to Smash.GG and inserting/push notifying
 *  based on new tournaments discovered. Main entry point of module. Walks over
 *  the endpoint in pages until it's determined that we've gotten pretty much everything. Yes, this
 *  could be slightly more efficient, but the scope of the entire project is small enough now that
 *  I'm happy to punt on that.
 */
pub fn fetch_and_store_tournaments(client: Client, db: PgConnection) -> Result<(), reqwest::Error> { 
    use schema::tournaments::dsl::*;

    let mut page = 1;
    let mut count = 0;
    let mut insertions: Vec<Tournament> = Vec::new();

    println!("> Fetching Tournaments...");
    loop {
        println!("    Fetching page {}", page);
        let api: &str = &api_url(page);
        let content: Response = client.get(api).send()?.json()?;

        for tournament in content.items.entities.tournament {
            count += 1;
            
            let tourney = Tournament {
                tournament_id: tournament.id,
                tournament_type: 1,
                name: tournament.name.clone(),
                url: build_tournament_url(&tournament).clone(),
                location: build_location_str(&tournament).clone(),
                details: util::sanitize_optional_string(tournament.details),
                hashtag: util::sanitize_optional_string(tournament.hashtag),
                games: collect_games(tournament.id, &content.items.entities.event),
                starts: NaiveDateTime::from_timestamp(tournament.startAt, 0),
                ends: NaiveDateTime::from_timestamp(tournament.endAt, 0),
                timezone: tournament.timezone.clone(),
                published: tournament.published
            };

            let filter = tournaments.filter(tournament_id.eq(tournament.id)).filter(tournament_type.eq(1));
            let existing: i64 = filter.count().get_result(&db).expect("Error counting existing tourneys?");
           
            if existing > 0 {
                println!("        Updating tournament {}", tournament.id);
                diesel::update(filter).set(&tourney);
            } else {
                insertions.push(tourney);
            }
        }

        if count >= content.total_count { break; }
        page += 1;
        util::sleep_for_random_seconds();
    }
    
    println!("> Completed, writing {} new tournaments", insertions.len());
    diesel::insert_into(tournaments).values(&insertions).execute(&db).expect("Error saving Smash.GG tournaments.");
    Ok(())
}
