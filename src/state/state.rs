use mongodb::Client;
use rocket::State;

pub struct MongoDatabaseState {
    pub client: Client,
}
