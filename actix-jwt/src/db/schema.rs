table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        email -> Text,
        created_at -> Timestamp,
        login_session -> Text,
    }
}
