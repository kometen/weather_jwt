table! {
    readings {
        measurement_time_default -> Timestamptz,
        id -> Integer,
        data -> Jsonb,
    }
}

table! {
    latest_readings {
        measurement_time_default -> Timestamptz,
        id -> Integer,
        data -> Jsonb,
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
