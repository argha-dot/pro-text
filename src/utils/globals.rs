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
        // TODO: make this access token or something
        password_hash: String,
    },
}

impl AuthState {
    pub fn is_logged_in(&self) -> bool {
        matches!(self, AuthState::LoggedIn { .. })
    }
}

pub fn auth_state() -> (Signal<AuthState>, SignalSetter<AuthState>) {
    let notes = use_context::<RwSignal<GlobalState>>().expect("global state to be provided");

    let (current_user, set_current_user) = create_slice(
        notes,
        |state: &GlobalState| state.auth.clone(),
        |state: &mut GlobalState, auth_state: AuthState| {
            state.auth = auth_state;
        },
    );

    (current_user, set_current_user)
}

pub fn is_logged_in() -> Signal<bool> {
    let global = use_context::<RwSignal<GlobalState>>().expect("global state to be provided");
    Signal::derive(move || matches!(global.get().auth, AuthState::LoggedIn { .. }))
}

pub fn get_username() -> Signal<Option<String>> {
    let global = use_context::<RwSignal<GlobalState>>().expect("global state to be provided");

    let (current_user, _) = create_slice(
        global,
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
    let global = use_context::<RwSignal<GlobalState>>().expect("global state to be provided");

    let (current_note, set_current_note) = create_slice(
        global,
        |state: &GlobalState| state.current_note.clone(),
        |state: &mut GlobalState, note_id: Option<String>| state.current_note = note_id,
    );

    (current_note, set_current_note)
}
