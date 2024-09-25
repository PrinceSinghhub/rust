mod connection;

use bson;
use connection::{connect_to_mongo, get_questions_collection};
use futures::TryStreamExt;
use serde_json::Value;
use warp::{Filter, Rejection};

#[derive(Debug)]
struct CustomError;
impl warp::reject::Reject for CustomError {}

#[tokio::main]
async fn main() {
    // Connect to MongoDB
    let client = connect_to_mongo()
        .await
        .expect("Failed to connect to MongoDB");
    let questions_collection = get_questions_collection(&client).await;

    // Define the endpoint
    let questions_route = warp::path("questions").and(warp::get()).and_then(move || {
        let questions_collection = questions_collection.clone();
        async move {
            match questions_collection.find(None, None).await {
                Ok(mut cursor) => {
                    let mut questions = Vec::new();
                    while let Ok(Some(document)) = cursor.try_next().await {
                        let json_document: Value =
                            bson::from_bson(bson::Bson::Document(document)).unwrap();
                        println!("{:#?}", json_document); // Print each document to console
                        questions.push(json_document);
                    }
                    Ok::<_, Rejection>(warp::reply::json(&questions))
                }
                Err(e) => {
                    eprintln!("Failed to fetch documents: {}", e);
                    Err(warp::reject::custom(CustomError))
                }
            }
        }
    });

    // Start the warp server
    warp::serve(questions_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
