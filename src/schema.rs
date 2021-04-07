table! {
    readings {
        measurement_time_default -> Timestamp,
        id -> Integer,
        index -> Integer,
        field_description -> Text,
        measurement -> Double,
    }
}

table! {
    locations {
        publication_time -> Timestamp,
        id -> Integer,
        name -> Text,
        latitude -> Numeric,
        longitude -> Numeric,
    }
}
