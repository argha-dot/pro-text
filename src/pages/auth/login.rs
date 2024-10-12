use leptos::*;
use leptos_dom::logging;
use leptos_router::use_navigate;

use crate::{
    api::login,
    utils::{auth_state, AuthState},
};

#[component]
pub fn LoginPage() -> impl IntoView {
    let password = create_rw_signal("".to_string());
    let user_name = create_rw_signal("".to_string());

    let (_, set_auth_state) = auth_state();
    let navigate = use_navigate();

    let login_action = create_action(move |_: &()| {
        let navigate = navigate.clone();

        async move {
            let resp = login(user_name.get()).await;

            match resp {
                Ok(user) => {
                    set_auth_state.set(AuthState::LoggedIn {
                        username: user.username,
                    });
                    navigate("/", Default::default());
                }
                Err(e) => {
                    logging::console_log(format!("{:?}", e).as_str());
                }
            }
        }
    });

    view! {
        <div class="w-full flex items-center justify-center">
            <form
                on:submit=move |ev| {
                    ev.prevent_default();
                    login_action.dispatch(());
                }
                class="login__form"
            >
                <h1>"LOGIN"</h1>

                <div class="login__component">
                    <label class=move || {
                        format!(
                            "login__form__input {}",
                            if !user_name.get().is_empty() { "has-text" } else { "" },
                        )
                    }>
                        <p>"Username"</p>
                        <input
                            type="text"
                            name="username"
                            prop:value=user_name
                            on:input=move |ev: ev::Event| {
                                user_name.set(event_target_value(&ev));
                            }
                        />
                    </label>

                    <label class=move || {
                        format!(
                            "login__form__input {}",
                            if !password.get().is_empty() { "has-text" } else { "" },
                        )
                    }>
                        <p>"Password"</p>
                        <input
                            type="password"
                            prop:value=password
                            name="password"
                            on:input=move |ev: ev::Event| {
                                password.set(event_target_value(&ev));
                            }
                        />
                    </label>

                    <button class="login__form__button" type="submit">
                        "LOGIN"
                    </button>
                </div>
            </form>
        </div>
    }
}
