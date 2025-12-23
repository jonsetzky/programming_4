diesel::table! {
    messages (id) {
        id -> Binary,
        channel -> Binary,
        sender -> Binary,
        time -> TimestamptzSqlite,
        message -> Text,
    }
}

diesel::table! {
    channels (id) {
        id -> Binary,
        name -> Text,
    }
}
