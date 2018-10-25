table! {
    channels (id) {
        id -> Int4,
        creator -> Varchar,
        member -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    messages (id) {
        id -> Int4,
        channel_id -> Int4,
        message -> Text,
        nonce -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(messages -> channels (channel_id));

allow_tables_to_appear_in_same_query!(channels, messages,);
