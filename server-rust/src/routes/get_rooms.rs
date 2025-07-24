use actix_web::{Error, HttpResponse, error, get, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
struct Room {
    name: String,
    description: String,
}

// New struct to represent the Room with its generated ID for the response
#[derive(Serialize, Deserialize)]
struct GetRoomsResponse {
    result: Vec<RoomReturned>,
}

#[derive(Serialize, Deserialize)]
struct RoomReturned {
    id: String,
    name: String,
    created_at: String,
    questionCount: i64,
}

impl RoomReturned {
    pub fn new(id: String, name: String, created_at: String, questionCount: i64) -> Self {
        Self {
            created_at,
            id,
            name,
            questionCount,
        }
    }
}

#[get("/rooms")]
async fn get_rooms(db: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let result = sqlx::query!(
        r#"
        SELECT rooms.id, rooms.name, rooms.created_at, COUNT(questions.id) AS questions_count
        FROM rooms
        LEFT JOIN questions ON questions.roomid = rooms.id
        GROUP BY rooms.id
        ORDER BY rooms.created_at
        "#
    )
    .fetch_all(db.get_ref())
    .await;

    if let Ok(record) = result {
        let mut response = Vec::new();

        for room in record {
            let question_count = room.questions_count.unwrap_or(0);
            response.push(RoomReturned::new(
                room.id.to_string(),
                room.name,
                room.created_at.to_string(),
                question_count,
            ));
        }

        Ok(HttpResponse::Created().json(response))
    } else {
        Err(error::ErrorBadRequest("Failed to create room"))
    }
}
