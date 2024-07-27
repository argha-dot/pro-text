use leptos::*;

#[derive(Debug, Clone)]
pub struct GlobalState {
    pub auth: AuthState,
    pub current_note: Option<String>,
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

pub fn set_current_note() -> (Signal<Option<String>>, SignalSetter<Option<String>>) {
    let notes = use_context::<RwSignal<GlobalState>>().expect("global state to be provided");

    let (current_note, set_current_note) = create_slice(
        notes,
        |state: &GlobalState| state.current_note.clone(),
        |state: &mut GlobalState, note_id: Option<String>| state.current_note = note_id,
    );

    (current_note, set_current_note)
}
