table! {
    ar_internal_metadata (key) {
        key -> Varchar,
        value -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    materiels (id) {
        id -> Int4,
        nom -> Nullable<Varchar>,
        description -> Nullable<Text>,
        photo -> Nullable<Varchar>,
        poids -> Nullable<Int4>,
        reforme -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    materiels_traces (id) {
        id -> Int8,
        trace_id -> Nullable<Int8>,
        materiel_id -> Nullable<Int8>,
    }
}

table! {
    schema_migrations (version) {
        version -> Varchar,
    }
}

table! {
    traces (id) {
        id -> Int8,
        traces_id -> Nullable<Int8>,
        titre -> Nullable<Varchar>,
        sous_titre -> Nullable<Varchar>,
        description -> Nullable<Text>,
        fichier_gpx -> Nullable<Varchar>,
        altitude_minimum -> Nullable<Int4>,
        altitude_maximum -> Nullable<Int4>,
        ascension_totale -> Nullable<Int4>,
        descente_totale -> Nullable<Int4>,
        heure_debut -> Nullable<Timestamp>,
        heure_fin -> Nullable<Timestamp>,
        distance_totale -> Nullable<Int4>,
        lat_depart -> Nullable<Numeric>,
        long_depart -> Nullable<Numeric>,
        lat_arrivee -> Nullable<Numeric>,
        long_arrivee -> Nullable<Numeric>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        repertoire_photos -> Nullable<Varchar>,
        moyen -> Nullable<Varchar>,
        polylines -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    ar_internal_metadata,
    materiels,
    materiels_traces,
    schema_migrations,
    traces,
);
