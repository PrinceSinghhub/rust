// connection.rs
use mongodb::bson::Document;
use mongodb::{error::Result, Client, Collection};

pub async fn connect_to_mongo() -> Result<Client> {
    let client_uri = "mongodb://localhost:27017/DSAQuestions";
    let client = Client::with_uri_str(client_uri).await?;
    Ok(client)
}

pub async fn get_questions_collection(client: &Client) -> Collection<Document> {
    let database = client.database("DSAQuestions");
    let collection = database.collection::<Document>("questions");
    collection
}
