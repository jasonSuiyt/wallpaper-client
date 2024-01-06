// @generated automatically by Diesel CLI.

diesel::table! {
    bing (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
        uhd_url -> Text,
        uhd_file_path -> Text,
        normal_file_path -> Text,
        source -> Text,
        created_date -> Date,
    }
}
