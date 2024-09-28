use crate::utils::get_username;

use leptos::*;
use leptos_router::*;

#[component]
pub fn UserSidebarCard() -> impl IntoView {
    let current_user = get_username();

    let navigate = use_navigate();
    if current_user.get().is_none() {
        navigate("/login", Default::default());
    }

    let user_msg = move || format!("Hello {}", current_user.get().unwrap_or("User".to_string()));

    view! {
        <section class="sidebar__user">
            <div class="sidebar__user__card">{user_msg}</div>
        </section>
    }
}
