use crate::components::Sidebar;
use leptos::*;
use leptos_router::*;

#[component]
pub fn NotesPage() -> impl IntoView {
    view! {
        <Sidebar />
        <Outlet />
    }
}
