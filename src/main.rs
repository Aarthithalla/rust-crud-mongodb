use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::{delete, post, put};
use axum::{routing::get, Json, Router};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::ClientOptions;
use mongodb::Client;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    age: i32,
}
#[tokio::main]
async fn main() {
    let mongodb_uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(mongodb_uri)
        .await
        .expect("failed to parse mongodb uri ");
    let client = Client::with_options(client_options).expect("failed to intialize mongodb client ");
    let app = Router::new()
        .route("/", post(create_person_details))
        .route("/", get(getpersondetails))
        .route("/", delete(deletepersondetails))
        .route("/", put(updatepersondetails));
    let listner = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
async fn create_person_details(Json(data): Json<Person>) -> impl IntoResponse {
    let uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(uri).await.expect("cannot parse uri");
    let client = Client::with_options(client_options).expect("failed to intialize mongodb client ");
    let my_coll: Collection<Person> = client.database("sampledb").collection("persondetails");
    let result = my_coll
        .insert_one(data)
        .await
        .expect("no insertion takes place ");
    Json(result)
}
async fn getpersondetails(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    dbg!("{}", &params);
    let uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(uri).await.expect("cannot parse uri");
    let client = Client::with_options(client_options).expect("failed to intialize mongodb client");
    let my_coll: Collection<Person> = client.database("sampledb").collection("persondetails");
    let mut data = Vec::new();
    if let Some(id) = params.get("id") {
        dbg!("recevied id is {}", &id);
        match ObjectId::parse_str(id) {
            Ok(objectid) => {
                let filter = doc! {"_id":objectid};
                dbg!("{}", &filter);
                let mut result = my_coll
                    .find(filter)
                    .await
                    .expect("failed to fetch person details");

                while let Some(info) = result.try_next().await.expect("failed to fetch ") {
                    data.push(info);
                }
            }
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }
    dbg!("{}", &data);

    Json(data)
}
async fn deletepersondetails(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(uri).await.expect("cannot parse uri");
    let client = Client::with_options(client_options).expect("failed to intialize mongodb client");
    let my_coll: Collection<Person> = client.database("sampledb").collection("persondetails");
    if let Some(id) = params.get("id") {
        match ObjectId::parse_str(id) {
            Ok(id) => {
                let filter = doc! {"_id":id};
                let result = my_coll
                    .delete_one(filter)
                    .await
                    .expect("failed to delete person details ");
            }
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }
    Json("person details deleted sucessfully".to_string())
}
async fn updatepersondetails(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(uri).await.expect("cannot parse uri");
    let client = Client::with_options(client_options).expect("failed to intialize mongodb client");
    let my_coll: Collection<Person> = client.database("sampledb").collection("persondetails");
    let update = params.get("name");
    if let Some(id) = params.get("id") {
        match ObjectId::parse_str(id) {
            Ok(id) => {
                let filter = doc! {"_id":id};
                let update = doc! {"$set":doc! {"name":update}};
                let result = my_coll
                    .update_one(filter, update)
                    .await
                    .expect("failed to update ");
            }
            Err(err) => {
                eprint!("{}", err);
            }
        }
    }
    Json("person details updated".to_string())
}
