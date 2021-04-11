table! {
    readings {
        measurement_time_default -> Timestamptz,
        id -> Integer,
        index -> Integer,
        field_description -> Text,
        measurement -> Numeric,
    }
}

table! {
    locations {
        publication_time -> Timestamptz,
        id -> Integer,
        name -> Text,
        latitude -> Numeric,
        longitude -> Numeric,
    }
}
