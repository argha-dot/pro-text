use leptos::*;
use leptos_query::*;

use crate::{
    api::{get_note, get_notes, get_notes_metadata},
    models::{Note, NoteMetadata},
};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllNotesTag;

pub fn get_all_notes_query() -> QueryScope<(AllNotesTag, String), Result<Vec<Note>, ServerFnError>>
{
    create_query(
        |(_, user_id)| async move { get_notes(user_id).await },
        QueryOptions {
            ..Default::default()
        },
    )
}

pub fn get_note_query() -> QueryScope<(String, String), Result<Note, ServerFnError>> {
    create_query(
        |(id, user_id)| async move { get_note(id, user_id).await },
        QueryOptions {
            ..Default::default()
        },
    )
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllNoteMetadatasTag;

pub fn get_all_note_metadatas_query(
) -> QueryScope<(AllNoteMetadatasTag, String), Result<Vec<NoteMetadata>, ServerFnError>> {
    create_query(
        |(_, user_id)| async move { get_notes_metadata(user_id).await },
        QueryOptions {
            ..Default::default()
        },
    )
}
