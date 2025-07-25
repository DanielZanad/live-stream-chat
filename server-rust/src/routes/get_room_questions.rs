use actix_web::{Error, HttpResponse, error, get, web};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::uuid};

// New struct to represent the Room with its generated ID for the response
#[derive(Serialize, Deserialize)]
struct Question {
    id: String,
    question: String,
    answer: String,
    created_at: String,
}

#[derive(Serialize, Deserialize)]
struct CreateRoomResponse {
    questions: Vec<Question>,
}

impl Question {
    pub fn new(id: String, question: String, answer: String, created_at: String) -> Self {
        Self {
            answer,
            created_at,
            id,
            question,
        }
    }
}

#[get("/rooms/{roomId}/questions")]
async fn get_room_questions(
    path: web::Path<(String,)>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let room_id = path.into_inner().0;
    let room_id = uuid::Uuid::parse_str(&room_id)
        .map_err(|_| error::ErrorBadRequest("Invalid roomId format"))?;

    let result = sqlx::query!(
        r#"
        SELECT id, question, answer , created_at
        FROM questions
        WHERE roomid = ($1)
        ORDER BY created_at DESC
        "#,
        room_id
    )
    .fetch_all(db.get_ref())
    .await;

    if let Ok(record) = result {
        let mut response = Vec::new();

        for question in record {
            if let Some(answer) = question.answer {
                response.push(Question::new(
                    question.id.to_string(),
                    question.question,
                    answer,
                    question.created_at.to_string(),
                ));
            } else {
                response.push(Question::new(
                    question.id.to_string(),
                    question.question,
                    String::new(),
                    question.created_at.to_string(),
                ));
            }
        }

        Ok(HttpResponse::Created().json(response))
    } else {
        Err(error::ErrorBadRequest("Failed to create room"))
    }
}
