table! {
    users (id) {
        id -> Uuid,
        user_id -> Text,
        nickname -> Text,
        face_url -> Text,
        gender -> Int4,
        phone_number -> Nullable<Text>,
        birth -> Int4,
        email -> Nullable<Text>,
        create_time -> Timestamptz,
        app_manager_level -> Nullable<Int4>,
        ex -> Text,
        attached_info -> Text,
        is_deleted -> Bool,
    }
}
