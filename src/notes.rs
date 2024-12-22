use chrono::Utc;
use serde::{Deserialize, Serialize};

// For actual Note and can be used for detail note
#[derive(Serialize)]
pub struct Note {
    pub id: uuid::Uuid,
    pub title: String,
    pub tags: Vec<String>,
    pub body: String,
    pub created_at: i64,
    pub updated_at: i64
}

impl Note {
    pub fn new(id: uuid::Uuid, payload: NotePayload, created_at: i64, updated_at: i64) -> Self {
        Note {
            id,
            title: payload.title,
            tags: payload.tags,
            body: payload.body,
            created_at,
            updated_at
        }
    }

    pub fn update(&mut self, payload: NotePayload) {
        self.title = payload.title;
        self.tags = payload.tags;
        self.body = payload.body;
        self.updated_at = Utc::now().timestamp();
    }
}

// For the "Get All Notes", since it will just display the summary
#[derive(Serialize)]
pub struct NoteSummary<'a> {
    pub id: uuid::Uuid,
    pub title: &'a String,
    pub tags: &'a Vec<String>,
    pub updated_at: String
}

// Payload that can be used for adding and updating/editing the note
#[derive(Deserialize)]
pub struct NotePayload {
    pub title: String,
    pub tags: Vec<String>,
    pub body: String
}
