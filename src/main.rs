#[macro_use]
extern crate rocket;
mod database;
mod routes;
use database::database::Database;
use dotenv::dotenv;
use mongodb::Client;

pub struct MongoDatabaseState {
    pub client: Client,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let connection_string = std::env::var("CONNECTION_STRING")
        .expect("CONNECTION_STRING must be set as an environment variable.");

    match Database::connect(connection_string).await {
        Ok(db) => {
            if let Some(client) = db.client {
                info!("Database connected successfully");

                let db_client = MongoDatabaseState { client: client };

                let _rocket: rocket::Rocket<rocket::Ignite> = rocket::build()
                    .mount(
                        "/environmental_data",
                        routes![routes::environmental_information::get_environmental_information],
                    )
                    .manage(db_client)
                    .launch()
                    .await?;
            } else {
                error!("Client is None after successful connection");
            }
        }
        Err(e) => {
            error!("Failed to connect to the database: {:?}", e);
        }
    }

    Ok(())
}
