use crate::{components::Sidebar, utils::is_logged_in};
use leptos::*;
use leptos_router::*;

#[component]
pub fn NotesPage() -> impl IntoView {
    let is_logged_in = is_logged_in();
    move || {
        if is_logged_in.get() {
            view! {
                <Sidebar />
                <Outlet />
            }
            .into_view()
        } else {
            view! {
                <div>"login"</div>
            }
            .into_view()
        }
    }
}
