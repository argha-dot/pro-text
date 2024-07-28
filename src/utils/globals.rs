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
        // The Password Stored Here never leaves the client machine, nor does it ever gets sent, hashed or otherwise to the server
        password: String,
    },
}

pub fn auth_state() -> (Signal<AuthState>, SignalSetter<AuthState>) {
    let notes = use_context::<RwSignal<GlobalState>>().expect("global state to be provided");

    let (current_user, set_current_user) = create_slice(
        notes,
        |state: &GlobalState| state.auth.clone(),
        |_: &mut GlobalState, _: AuthState| {},
    );

    (current_user, set_current_user)
}

pub fn get_username() -> Signal<Option<String>> {
    let notes = use_context::<RwSignal<GlobalState>>().expect("global state to be provided");

    let (current_user, _) = create_slice(
        notes,
        |state: &GlobalState| {
            if let AuthState::LoggedIn { username, .. } = state.auth.clone() {
                Some(username)
            } else {
                None
            }
        },
        |_: &mut GlobalState, _: AuthState| {},
    );

    current_user
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
