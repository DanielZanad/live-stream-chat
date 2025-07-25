use actix_web::{Error, HttpResponse, error, post, web};
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::uuid};

use crate::env::get_env_var;

#[derive(Serialize, Deserialize)]
struct Question {
    question: String,
}

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

#[derive(Debug, Deserialize)]
struct EmbeddingsResponse {
    embeddings: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct AnswerResponse {
    answer: String,
}

impl CreateRoomResponse {
    pub fn new(question_id: String) -> Self {
        Self {
            questionId: question_id,
        }
    }
}

const MAX_SIZE: usize = 262_144;

#[post("/rooms/{roomId}/questions")]
async fn create_question(
    mut payload: web::Payload,
    path: web::Path<(String,)>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let client = Client::new();
    let api_url = get_env_var("GEMINI_API_URL");

    let room_id = uuid::Uuid::parse_str(&path.into_inner().0)
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
    let question = serde_json::json!({ "question": obj.question });

    let response = client
        .post(format!("{}/audio/embeddings", api_url))
        .json(&question)
        .send()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed to generate embeddings"))?;

    let response: EmbeddingsResponse = response.json().await.map_err(|error| {
        println!("{:?}", error);
        return error::ErrorBadRequest("Invalid response from transcription API");
    })?;

    let embeddings_as_string = format!(
        "[{}]",
        response
            .embeddings
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    let chunks = sqlx::query!(
        r#"
        SELECT id, transcription, roomid, 1 - (embeddings <=> CAST($1 AS vector)) AS similarity
        FROM audio_chunks
        WHERE
            roomid = $2
            AND 1 - (embeddings <=> CAST($1 AS vector)) > 0.7
        ORDER BY
            embeddings <=> CAST($1 AS vector)
        "#,
        embeddings_as_string as _,
        room_id
    )
    .fetch_all(db.get_ref())
    .await
    .map_err(|error| {
        println!("{:?}", error);
        return error::ErrorBadRequest("Failed to query audio chunks");
    })?;

    if chunks.is_empty() {
        return Err(error::ErrorBadRequest("No similar chunks found"));
    }

    let transcriptions: Vec<String> = chunks
        .into_iter()
        .map(|chunk| chunk.transcription)
        .collect();

    let payload = serde_json::json!({
        "question": obj.question,
        "transcriptions": transcriptions,
    });

    let answer_response = client
        .post(format!("{}/audio/generate/answer", api_url))
        .json(&payload)
        .send()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed to generate answer"))?;

    let response: AnswerResponse = answer_response
        .json()
        .await
        .map_err(|_| error::ErrorBadRequest("Failed to generate embeddings"))?;

    let result = sqlx::query!(
        "INSERT INTO questions (roomid, question, answer) VALUES ($1, $2, $3) RETURNING roomid",
        room_id,
        obj.question,
        response.answer
    )
    .fetch_one(db.get_ref())
    .await
    .map_err(|_| error::ErrorBadRequest("Failed to create question"))?;

    let room_id = result
        .roomid
        .ok_or_else(|| error::ErrorBadRequest("Missing room ID in insert result"))?;

    Ok(HttpResponse::Created().json(CreateRoomResponse::new(room_id.to_string())))
}
