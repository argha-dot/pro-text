use leptos::*;

use crate::models::{Note, NoteMetadata};

#[server]
pub async fn get_notes() -> Result<Vec<Note>, ServerFnError> {
    use crate::utils::ssr::*;
    use futures::TryStreamExt;

    let mut conn = db().await?;

    let mut notes = Vec::new();
    let mut rows = sqlx::query_as::<_, Note>("SELECT * FROM notes").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        notes.push(row);
    }

    Ok(notes)
}

#[server(GetNoteMetada)]
pub async fn get_notes_metadata() -> Result<Vec<NoteMetadata>, ServerFnError> {
    use crate::utils::ssr::*;
    use futures::TryStreamExt;

    let mut conn = db().await?;

    let mut note_metadata: Vec<NoteMetadata> = Vec::new();
    let mut rows = sqlx::query_as::<_, NoteMetadata>("SELECT id,title FROM notes").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        note_metadata.push(row);
    }

    Ok(note_metadata)
}

#[server(AddNote)]
pub async fn add_note(title: String) -> Result<(), ServerFnError> {
    use crate::utils::ssr::*;
    use uuid::Uuid;

    let mut conn = db().await?;
    let uid = Uuid::new_v4().to_string();

    match sqlx::query("INSERT INTO notes (id,title,note) VALUES ($1, $2, '')")
        .bind(uid)
        .bind(title)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
