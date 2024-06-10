use chrono::prelude::*;
use mongodb::{
    bson::{doc, DateTime as BsonDateTime},
    options::FindOneOptions,
    Collection,
};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::MongoDatabaseState;

#[derive(Serialize, Deserialize, Debug)]
struct ClimateMonitoring {
    device_id: String,
    interval_start: BsonDateTime,
    data: Vec<ClimateData>,
    units: Units,
    metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClimateData {
    timestamp: BsonDateTime,
    temperature: f64,
    humidity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Units {
    temperature: String,
    humidity: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    sensor_type: String,
    installation_date: BsonDateTime,
}

async fn check_interval_start_in_last_120_minutes(
    collection: Collection<ClimateMonitoring>,
) -> mongodb::error::Result<(bool, Option<ClimateMonitoring>)> {
    let current_time: DateTime<Utc> = Utc::now();
    let time_120_minutes_ago: DateTime<Utc> = current_time - chrono::Duration::minutes(120);

    let current_time_millis: i64 = current_time.timestamp_millis();
    let time_120_minutes_ago_millis: i64 = time_120_minutes_ago.timestamp_millis();

    let filter: mongodb::bson::Document = doc! {
        "interval_start": {
            "$gte": BsonDateTime::from_millis(time_120_minutes_ago_millis),
            "$lte": BsonDateTime::from_millis(current_time_millis),
        }
    };

    let find_options = FindOneOptions::builder()
        .sort(doc! { "interval_start": -1 })
        .build();

    let result: Option<ClimateMonitoring> = collection.find_one(filter, find_options).await?;

    let is_within_last_120_minutes = result.is_some();

    Ok((is_within_last_120_minutes, result))
}

#[get("/")]
pub async fn get_environmental_information(
    db_client: &State<MongoDatabaseState>,
) -> status::Custom<content::RawJson<String>> {
    let collection: Collection<ClimateMonitoring> = db_client
        .client
        .database("gecko-client")
        .collection("environmental_information");

    match check_interval_start_in_last_120_minutes(collection.clone()).await {
        Ok((_is_within_last_120_minutes, result)) => {
            let json_string: String = to_string(&result).unwrap();
            status::Custom(Status::Ok, content::RawJson(json_string))
        }
        Err(e) => {
            let error_message = format!("Failed to query the database: {:?}", e);
            let json_error = to_string(&error_message).unwrap();
            status::Custom(Status::InternalServerError, content::RawJson(json_error))
        }
    }
}
