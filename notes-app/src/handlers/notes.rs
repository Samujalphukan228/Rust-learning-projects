use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use futures::stream::TryStreamExt;
use crate::{models::Note, error::AppError};

type Db = Database;

pub async fn create_note(
    State(db): State<Db>,
    Json(mut note): Json<Note>,
) -> Result<Json<Note>, AppError> {
    let collection = db.collection::<Note>("notes");
    let result = collection.insert_one(&note, None).await?;
    note.id = result.inserted_id.as_object_id();
    Ok(Json(note))
}

pub async fn get_notes(
    State(db): State<Db>,
) -> Result<Json<Vec<Note>>, AppError> {
    let collection = db.collection::<Note>("notes");
    let mut cursor = collection.find(None, None).await?;

    let mut notes = Vec::new();
    while let Some(note) = cursor.try_next().await? {
        notes.push(note);
    }

    Ok(Json(notes))
}

pub async fn get_note(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> Result<Json<Note>, AppError> {
    let obj_id = ObjectId::parse_str(&id)
        .map_err(|_| AppError::BadRequest)?;

    let collection = db.collection::<Note>("notes");
    let note = collection
        .find_one(doc! { "_id": obj_id }, None)
        .await?
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(note))
}

pub async fn update_note(
    State(db): State<Db>,
    Path(id): Path<String>,
    Json(note): Json<Note>,
) -> Result<(), AppError> {
    let obj_id = ObjectId::parse_str(&id)
        .map_err(|_| AppError::BadRequest)?;

    let collection = db.collection::<Note>("notes");
    let result = collection
        .update_one(
            doc! { "_id": obj_id },
            doc! { "$set": { "title": note.title, "content": note.content } },
            None,
        )
        .await?;

    if result.matched_count == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

pub async fn delete_note(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    let obj_id = ObjectId::parse_str(&id)
        .map_err(|_| AppError::BadRequest)?;

    let collection = db.collection::<Note>("notes");
    let result = collection
        .delete_one(doc! { "_id": obj_id }, None)
        .await?;

    if result.deleted_count == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}