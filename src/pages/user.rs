use leptos::*;
use leptos_router::*;

use crate::components::Sidebar;

#[component]
pub fn UserPage() -> impl IntoView {
    view! {
        <Sidebar />
        <Outlet />
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    view! {
        <div class="text-white">"this moi"</div>
    }
}
