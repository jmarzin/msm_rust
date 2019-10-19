use diesel::pg::types::date_and_time::PgTimestamp;
use diesel::pg::types::floats::PgNumeric;


#[derive(Queryable)]
pub struct traces {
    id: i64,
    traces_id: i64,
    titre: String,
    sous_titre: String,
    description: String,
    fichier_gpx: String,
    altitude_minimum: i32,
    altitude_maximum: i32,
    ascension_totale: i32,
    descente_totale: i32,
    heure_debut: PgTimestamp,
    heure_fin: PgTimestamp,
    distance_totale: i32,
    lat_depart: PgNumeric,
    long_depart: PgNumeric,
    lat_arrivee: PgNumeric,
    long_arrivee: PgNumeric,
    type_: String,
    created_at: PgTimestamp,
    updated_at: PgTimestamp,
    repertoire_photos: String,
    moyen: String,
    polylines: String
}

#[derive(Queryable)]
pub struct traces_1 {
    id: i64,
    traces_id: i64,
    titre: String,
    sous_titre: String,
    fichier_gpx: String,
    ascension_totale: i32,
    descente_totale: i32,
    heure_debut: PgTimestamp,
    distance_totale: i32,
    lat_depart: PgNumeric,
    long_depart: PgNumeric,
    lat_arrivee: PgNumeric,
    long_arrivee: PgNumeric,
    type_: String,
    moyen: String
}