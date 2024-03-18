use crate::components::Sidebar;
use crate::utils::get_all_note_metadatas;
use crate::utils::AuthState;
use crate::utils::GlobalState;
use crate::utils::NotesState;
use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let note_state = create_rw_signal(GlobalState {
        auth: AuthState::LoggedOut,
        notes_state: NotesState {
            note_metadatas: get_all_note_metadatas(),
            current_note: None,
        },
    });

    provide_context(note_state);

    view! {
        <main class="min-h-screen w-full bg-primary flex">
            <Sidebar />
            <Outlet />
        </main>
    }
}
