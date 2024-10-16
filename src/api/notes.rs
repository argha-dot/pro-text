use leptos::*;

use crate::models::{Note, NoteMetadata};

#[server]
pub async fn get_notes(user_id: String) -> Result<Vec<Note>, ServerFnError> {
    use crate::utils::ssr::*;
    use futures::TryStreamExt;

    let mut conn = db().await?;

    let mut notes = Vec::new();
    let mut rows = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE user_id = $1")
        .bind(user_id)
        .fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        notes.push(row);
    }

    Ok(notes)
}

#[server]
pub async fn get_note(id: String, user_id: String) -> Result<Note, ServerFnError> {
    use crate::utils::ssr::*;

    let mut conn = db().await?;
    let row = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .fetch_one(&mut conn)
        .await;

    match row {
        Ok(row) => {
            logging::log!("{:?}", row);
            Ok(row)
        }
        Err(e) => {
            logging::log!("{:?}", e);
            Err(ServerFnError::ServerError(e.to_string()))
        }
    }
    // while let Some()
}

#[server(GetNoteMetada)]
pub async fn get_notes_metadata(user_id: String) -> Result<Vec<NoteMetadata>, ServerFnError> {
    use crate::utils::ssr::*;
    use futures::TryStreamExt;

    let mut conn = db().await?;

    let mut note_metadata: Vec<NoteMetadata> = Vec::new();
    let mut rows =
        sqlx::query_as::<_, NoteMetadata>("SELECT id,title FROM notes WHERE user_id = $1")
            .bind(user_id)
            .fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        note_metadata.push(row);
    }

    Ok(note_metadata)
}

#[server(AddNote)]
pub async fn add_note(title: String, user_id: String) -> Result<(), ServerFnError> {
    use crate::utils::ssr::*;
    use leptos_axum::redirect;
    use uuid::Uuid;

    let mut conn = db().await?;
    let uid = Uuid::new_v4().to_string();

    match sqlx::query("INSERT INTO notes (id,title,note,user_id) VALUES ($1, $2, '', $3)")
        .bind(uid.clone())
        .bind(title)
        .bind(&user_id)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => {
            redirect(format!("{}/note/{}", &user_id, uid.clone()).as_str());
            Ok(())
        }
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(UpdateNote)]
pub async fn update_note(
    id: String,
    title: String,
    note: String,
    user_id: String,
) -> Result<(), ServerFnError> {
    use crate::utils::ssr::*;

    let mut conn = db().await?;

    match sqlx::query("UPDATE notes SET title = $1,note = $2 WHERE id = $3 AND user_id = $4")
        .bind(title)
        .bind(note)
        .bind(id)
        .bind(user_id)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(DeleteNote)]
pub async fn delete_note(id: String, user_id: String) -> Result<(), ServerFnError> {
    use crate::utils::ssr::*;

    let mut conn = db().await?;

    match sqlx::query("DELETE FROM notes WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(&user_id)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => {
            leptos_axum::redirect(format!("/{}", user_id.clone()).as_str());
            Ok(())
        }
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
