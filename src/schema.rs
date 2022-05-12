table! {
    users (user_id) {
        user_id -> Uuid,
        nick_name -> Text,
        face_url -> Text,
        gender -> Int4,
        phone_number -> Text,
        birth -> Int4,
        email -> Text,
        create_time -> Timestamptz,
        app_manger_level -> Int4,
        ex -> Text,
        attached_info -> Text,
        is_deleted -> Bool,
    }
}
