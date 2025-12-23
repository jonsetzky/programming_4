diesel::table! {
    messages (id) {
        id -> Binary,
        channel -> Binary,
        sender -> Binary,
        time -> Time,
        message -> Text,
    }
}
