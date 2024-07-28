use crate::components::Sidebar;
use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="min-h-screen w-full bg-primary flex">
            <Sidebar />
            <Outlet />
        </main>
    }
}
