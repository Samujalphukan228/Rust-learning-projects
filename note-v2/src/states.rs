use mongodb::Database;
use crate::repository::NoteRepository;

#[derive(Clone)]
pub struct AppState {
    pub note_repo: NoteRepository,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        Self {
            note_repo: NoteRepository::new(&db),
        }
    }
}
