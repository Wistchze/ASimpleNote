use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::server_error::ServerError;
use crate::utils::convert_to_datetime;
use crate::AppState;
use crate::notes::{NotePayload, Note, NoteSummary};

pub async fn create_note(
    data: web::Data<AppState>, 
    payload: web::Json<NotePayload>,
) -> Result<HttpResponse, ServerError> {
    // Creating data
    let id = Uuid::new_v4();

    let now = Utc::now().timestamp();
    let created_at = now;
    let updated_at = now;

    // Getting data
    let payload = payload.into_inner();

    // Add the new note into data
    let new_note = Note::new(id, payload, created_at, updated_at);

    let mut notes = data.notes.lock().unwrap();
    notes.push(new_note);

    // Verify
    notes
        .iter()
        .find(|note| note.id == id)
        .ok_or_else(|| ServerError::InternalError(String::from("Unexpected error happened!")))?;

    Ok(HttpResponse::Created().json(json!({
        "status": "success",
        "message": "Successfully adding note!",
        "id": id
    })))
}

pub async fn get_notes(data: web::Data<AppState>) -> impl Responder {
    // Getting Data
    let notes = data.notes.lock().unwrap();

    // Show only needed fields
    let note_summaries: Vec<NoteSummary> = notes.iter().map(|note| NoteSummary {
        id: note.id,
        title: &note.title,
        tags: &note.tags,
        updated_at: convert_to_datetime(note.updated_at),
    }).collect();

    // Response
    HttpResponse::Ok().json(json!({
        "status": "success",
        "data": note_summaries
    }))
}

pub async fn get_note_by_id(
    data: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, ServerError> {
    // Params
    let id = path.into_inner();

    // Getting Data
    let notes = data.notes.lock().unwrap();
    let note = notes
        .iter()
        .find(|note| note.id == id)
        .ok_or_else(|| ServerError::NotFoundError(format!("Note with id {id} is not found!")))?;

    // Response
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": json!({
            "id": note.id,
            "title": note.title,
            "tags": note.tags,
            "body": note.body,
            "created_at": convert_to_datetime(note.created_at),
            "updated_at": convert_to_datetime(note.updated_at)
        })
    })))
}

pub async fn update_note_by_id(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
    payload: web::Json<NotePayload>,
) -> Result<HttpResponse, ServerError> {
    // Payload and Params
    let id = path.into_inner();
    let payload = payload.into_inner();

    // Getting the note that needs to be updated
    let mut notes = data.notes.lock().unwrap();
    let note = notes
        .iter_mut()
        .find(|note| note.id == id)
        .ok_or_else(|| ServerError::NotFoundError(format!("Unable to update, note with id {id} is not found!")))?;

    // Update the note
    note.update(payload);

    // Response
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Successfully update note!"
    })))
}

pub async fn delete_note_by_id(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>
) -> Result<HttpResponse, ServerError> {
    // Params
    let id = path.into_inner();

    // Get the data
    let mut notes = data.notes.lock().unwrap();
    let delete_note_index = notes
        .iter()
        .position(|note| note.id == id)
        .ok_or_else(|| ServerError::NotFoundError(format!("Cannot delete note of id {id}. Note is not found!")))?;

    // Delete
    notes.remove(delete_note_index);

    // Response
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Successfully delete note!"
    })))
}
