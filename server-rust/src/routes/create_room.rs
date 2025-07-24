use actix_web::{Error, HttpResponse, error, post, web};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
struct Room {
    name: String,
    description: String,
}

// New struct to represent the Room with its generated ID for the response
#[derive(Serialize, Deserialize)]
struct RoomWithId {
    id: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct CreateRoomResponse {
    roomId: String,
}

impl CreateRoomResponse {
    pub fn new(id: &str) -> Self {
        Self {
            roomId: id.to_string(),
        }
    }
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("/rooms")]
async fn create_room(
    db: web::Data<PgPool>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let obj = serde_json::from_slice::<Room>(&body)?;

    let result = sqlx::query!(
        "INSERT INTO rooms (name, description) VALUES ($1, $2) RETURNING id",
        obj.name,
        obj.description
    )
    .fetch_one(db.get_ref())
    .await;

    if let Ok(record) = result {
        Ok(HttpResponse::Created().json(CreateRoomResponse::new(&record.id.to_string())))
    } else {
        Err(error::ErrorBadRequest("Failed to create room"))
    }
}
