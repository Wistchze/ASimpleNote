use actix_web::web;

use crate::handler::{create_note, delete_note_by_id, get_note_by_id, get_notes, update_note_by_id};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/notes", web::post().to(create_note))
            .route("/notes", web::get().to(get_notes))
            .route("/notes/{id}", web::get().to(get_note_by_id))
            .route("/notes/{id}", web::put().to(update_note_by_id))
            .route("/notes/{id}", web::delete().to(delete_note_by_id))
    );
}