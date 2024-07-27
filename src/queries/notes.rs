use leptos::*;
use leptos_query::*;

use crate::{
    api::{get_note, get_notes, get_notes_metadata},
    models::{Note, NoteMetadata},
};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllNotesTag;

pub fn get_all_notes_query() -> QueryScope<AllNotesTag, Result<Vec<Note>, ServerFnError>> {
    create_query(
        |_| async move { get_notes().await },
        QueryOptions {
            ..Default::default()
        },
    )
}

pub fn get_note_query() -> QueryScope<String, Result<Note, ServerFnError>> {
    create_query(
        get_note,
        QueryOptions {
            ..Default::default()
        },
    )
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct AllNoteMetadatasTag;

pub fn get_all_note_metadatas_query(
) -> QueryScope<AllNoteMetadatasTag, Result<Vec<NoteMetadata>, ServerFnError>> {
    create_query(
        |_| async move { get_notes_metadata().await },
        QueryOptions {
            ..Default::default()
        },
    )
}
