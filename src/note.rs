use crate::components::NoteMain;
// use crate::components::Sidebar;
use crate::components::Sidebar;
use crate::utils::get_all_note_metadatas;
use crate::utils::AuthState;
use crate::utils::GlobalState;
use crate::utils::NotesState;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let note_state = create_rw_signal(GlobalState {
        auth: AuthState::LoggedOut,
        notes_state: NotesState {
            note_metadatas: get_all_note_metadatas(),
            current_note: None,
        },
    });

    // let notes = get_all_note_metadatas();

    provide_context(note_state);

    view! {
        <main class="min-h-screen w-full bg-primary flex">
            <Sidebar />
            <NoteMain />
        </main>
    }
}
