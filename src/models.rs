//! Models used for inserting and querying the database.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/24/2018

use chrono;
use schema::tournaments;

#[derive(Insertable, Queryable, AsChangeset, Debug)]
pub struct Tournament {
    pub tournament_id: i32,
    pub tournament_type: i32,
    pub name: String,
    pub details: String,
    pub location: String,
    pub url: String,
    pub hashtag: String,
    pub games: String,
    pub starts: chrono::NaiveDateTime,
    pub ends: chrono::NaiveDateTime,
    pub timezone: String,
    pub published: bool
}
