use actix_web::{Error, HttpResponse, error, post, web};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::uuid};

#[derive(Serialize, Deserialize)]
struct Question {
    question: String,
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
    questionId: String,
}

impl CreateRoomResponse {
    pub fn new(question_id: String) -> Self {
        Self {
            questionId: question_id,
        }
    }
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("/rooms/{roomId}/questions")]
async fn create_question(
    mut payload: web::Payload,
    path: web::Path<(String,)>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let room_id = path.into_inner().0;
    let room_id = uuid::Uuid::parse_str(&room_id)
        .map_err(|_| error::ErrorBadRequest("Invalid roomId format"))?;
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let obj = serde_json::from_slice::<Question>(&body)?;

    let result = sqlx::query!(
        "INSERT INTO questions (question, roomid) VALUES ($1, $2) RETURNING roomid",
        obj.question,
        room_id
    )
    .fetch_one(db.get_ref())
    .await;

    if let Ok(record) = result {
        if let Some(room_id) = record.roomid {
            Ok(HttpResponse::Created().json(CreateRoomResponse::new(room_id.to_string())))
        } else {
            Err(error::ErrorBadRequest("Failed to create room"))
        }
    } else {
        Err(error::ErrorBadRequest("Failed to create room"))
    }
}
