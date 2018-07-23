table! {
    tournaments (id) {
        id -> Int4,
        tournament_id -> Int4,
        tournament_type -> Int4,
        name -> Text,
        details -> Text,
        location -> Text,
        url -> Text,
        hashtag -> Text,
        games -> Text,
        starts -> Timestamp,
        ends -> Timestamp,
        timezone -> Text,
        published -> Bool,
        added -> Timestamptz,
    }
}
