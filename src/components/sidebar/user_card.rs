use crate::utils::get_username;

use leptos::*;

#[component]
pub fn UserSidebarCard() -> impl IntoView {
    let current_user = get_username();

    let user_msg = move || format!("Hello {}", current_user.get().unwrap_or("User".to_string()));

    view! {
        <section class="sidebar__user">
            <div class="sidebar__user__card">{user_msg}</div>
        </section>
    }
}
