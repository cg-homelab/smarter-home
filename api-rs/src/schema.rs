diesel::table! {
    users(id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Timestamptz,
        first_name -> Text,
        last_name -> Text,
        username -> Text,
        email -> Text,
        password -> Text
    }
}

diesel::table! {
    homes(id) {
        id -> Uuid,
        name -> Text
    }
}
