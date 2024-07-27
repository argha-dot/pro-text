use crate::components::Sidebar;
use crate::utils::{AuthState, GlobalState};
use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let note_state = create_rw_signal(GlobalState {
        auth: AuthState::LoggedOut,
        current_note: None,
    });

    provide_context(note_state);

    view! {
        <main class="min-h-screen w-full bg-primary flex">
            <Sidebar />
            <Outlet />
        </main>
    }
}
