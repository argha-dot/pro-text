use leptos::create_resource;
use leptos::*;

use crate::api::get_notes_metadata;
use crate::models::NoteMetadata;

#[derive(Debug, Clone)]
pub struct GlobalState {
    pub auth: AuthState,
    pub notes_state: NotesState,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum AuthState {
    #[default]
    LoggedOut,
    LoggedIn {
        username: String,
        username_hash: String,
    },
}

pub type NotesResuorceT = Resource<(), Result<Vec<NoteMetadata>, ServerFnError>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotesState {
    pub note_metadatas: NotesResuorceT,
    pub current_note: Option<String>,
}

pub fn get_all_note_metadatas() -> NotesResuorceT {
    let notes = create_resource(|| (), move |_| get_notes_metadata());

    notes
}

pub fn use_get_note_metadatas() -> (Signal<NotesState>, SignalSetter<NotesResuorceT>) {
    let notes = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");

    let (notes, set_notes) = create_slice(
        notes,
        |state: &GlobalState| state.notes_state.clone(),
        |state: &mut GlobalState, new: NotesResuorceT| {
            state.notes_state = NotesState {
                note_metadatas: new,
                current_note: state.notes_state.clone().current_note,
            }
        },
    );

    (notes, set_notes)
}

pub fn set_current_note() -> SignalSetter<Option<String>> {
    let notes = use_context::<RwSignal<GlobalState>>().expect("global state to be provided");

    let set_notes = create_write_slice(notes, |state, note_id| {
        state.notes_state = NotesState {
            note_metadatas: state.notes_state.clone().note_metadatas,
            current_note: note_id,
        }
    });

    set_notes
}
