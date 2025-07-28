use std::io::Read;

use actix_multipart::form::{MultipartForm, json::Json as MPJson, tempfile::TempFile};

use actix_web::{Error, HttpResponse, error, post, web};
use base64::{Engine, engine::general_purpose};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::uuid};

use crate::env::get_env_var;

#[derive(Serialize)]
struct OutgoingPayload {
    mime_type: String,
    audio_as_base_64: String,
}

impl OutgoingPayload {
    fn new(mime_type: String, audio_as_base_64: String) -> Self {
        Self {
            mime_type,
            audio_as_base_64,
        }
    }
}

#[derive(Debug, Deserialize)]
struct TranscriptionResponse {
    transcription: String,
    embeddings: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
    json: MPJson<Metadata>,
}

#[derive(Serialize, Deserialize)]
struct UploadAudioResponse {
    chunkId: String,
}

impl UploadAudioResponse {
    pub fn new(chunk_id: &str) -> Self {
        Self {
            chunkId: chunk_id.to_string(),
        }
    }
}

#[post("/rooms/{roomId}/audio")]
pub async fn upload_audio(
    chunkId: web::Path<(String,)>,
    db: web::Data<PgPool>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<HttpResponse, Error> {
    let room_id = chunkId.into_inner().0;
    let room_id = uuid::Uuid::parse_str(&room_id)
        .map_err(|_| error::ErrorBadRequest("Invalid roomId format"))?;
    let mut audio = form.file.file;

    let mut buffer = Vec::new();
    if let Err(e) = audio.read_to_end(&mut buffer) {
        return Err(error::ErrorInternalServerError(format!(
            "Failed to read file: {e}"
        )));
    }

    let base64_audio = general_purpose::STANDARD.encode(&buffer);

    let api_url = get_env_var("GEMINI_API_URL");
    let client = Client::new();

    let payload = OutgoingPayload::new("audio/webm".to_string(), base64_audio);
    let payload = serde_json::json!({
        "mimeType": payload.mime_type,
        "audioAsBase64": payload.audio_as_base_64,
    });

    let res = client
        .post(format!("{}/audio/transcribe", api_url))
        .json(&payload)
        .send()
        .await
        .map_err(|_| error::ErrorBadRequest("Error when transcribing audio"))?;

    let result: TranscriptionResponse = res
        .json()
        .await
        .map_err(|_| error::ErrorBadRequest("Invalid response from transcription API"))?;

    let result = sqlx::query!(
        "INSERT INTO audio_chunks (roomid, transcription, embeddings) VALUES ($1, $2, $3) RETURNING id",
        room_id,
        result.transcription,
        result.embeddings as _,
    )
    .fetch_one(db.get_ref())
    .await
    .map_err(|_| error::ErrorBadRequest("Failed to create audio_chunk"))?;

    let response = UploadAudioResponse::new(&result.id.to_string());

    Ok(HttpResponse::Created().json(response.chunkId))
}
