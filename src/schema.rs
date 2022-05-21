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

table! {
    friend_requests (from_user_id, to_user_id) {
        from_user_id -> Text,
        to_user_id -> Text,
        handle_result -> Int4,
        req_msg -> Text,
        create_time -> Timestamptz,
        handler_user_id -> Text,
        handle_msg -> Text,
        handle_time -> Timestamptz,
        ex -> Text,
        is_deleted -> Bool,
    }
}
