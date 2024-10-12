use ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

use crate::{
    api::{check_user_exists, create_user, login},
    components::Sidebar,
    utils::{auth_state, AuthState, UserPageParams},
};

#[derive(Copy, Clone, PartialEq, Eq)]
enum UserFormMode {
    Create,
    Login,
}

#[component]
fn UserForm(
    /// The Username
    username: Signal<String>,
    /// Mode of the form
    mode: UserFormMode,
) -> impl IntoView {
    let password = create_rw_signal(String::new());
    let (_, set_auth_state) = auth_state();

    let label_class = move || {
        format!(
            "login__form__input {}",
            if !password.get().is_empty() {
                "has-text"
            } else {
                ""
            }
        )
    };

    let create_user_action = create_action(move |(username, password): &(String, String)| {
        let username = username.to_string();
        let password = password.to_string();

        async move {
            let resp = create_user(username.to_string(), password.to_string()).await;
            match resp {
                Ok(()) => set_auth_state.set(AuthState::LoggedIn {
                    username: username.clone(),
                    password_hash: password.clone(),
                }),
                Err(_) => {}
            }
        }
    });

    let login_action = create_action(move |(username, password): &(String, String)| {
        let username = username.to_string();
        let password = password.to_string();

        async move {
            let resp = login(username.to_string(), password.to_string()).await;
            match resp {
                Ok(_) => set_auth_state.set(AuthState::LoggedIn {
                    username: username.clone(),
                    password_hash: password.clone(),
                }),
                Err(_) => {}
            }
        }
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if mode == UserFormMode::Login {
            logging::log!("Login");
            login_action.dispatch((username.get(), password.get()));
        } else {
            logging::log!("Create User");
            create_user_action.dispatch((username.get(), password.get()));
        }
    };

    view! {
        <Show
            when=move || matches!(mode, UserFormMode::Create)
        >
            <div class="text-white">"User does not exist"</div>
        </Show>
        <Form
            action=""
            method="POST"
            class="login__form"
            on:submit=on_submit
        >
            <h3>
                {
                    match mode {
                        UserFormMode::Create => "Create User",
                        UserFormMode::Login => "Login",
                    }
                }
            </h3>
            <label
                html_for="password"
                class={label_class}
            >
                <p>"Password"</p>
                <input
                    type="password"
                    name="user"
                    id="password"
                    required={true}
                    value={password}
                    on:input=move |ev| {
                        password.set(event_target_value(&ev));
                    }
                />
            </label>
            <button
                type="submit"
            >
                {
                    match mode {
                        UserFormMode::Create => "CREATE USER",
                        UserFormMode::Login => "LOGIN",
                    }
                }
            </button>
        </Form>
    }
}

#[component]
fn LoggedOutView() -> impl IntoView {
    let params = use_params::<UserPageParams>();
    let username = Signal::derive(move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.username.clone())
                .expect("Username must be found")
        })
    });

    let user_exists_resource = create_resource(
        move || username.get(),
        move |username| async move { check_user_exists(username).await },
    );

    let page = move || match user_exists_resource.get() {
        None => view! {
            <div class="text-white">"Loading..."</div>
        }
        .into_view(),
        Some(Ok(true)) => view! {
            <section
                class="flex flex-col items-center justify-center w-full min-h-screen py-2"
            >
                <UserForm
                    mode=UserFormMode::Login
                    username=username
                />
            </section>

        }
        .into_view(),
        Some(Ok(false)) => view! {
            <section
                class="flex flex-col items-center justify-center w-full min-h-screen py-2"
            >
                <UserForm
                    mode=UserFormMode::Create
                    username=username
                />
            </section>
        }
        .into_view(),
        Some(Err(err)) => view! {
            <section
                class="flex flex-col items-center justify-center w-full min-h-screen py-2"
            >
                <div class="text-white">"Error: "{err.to_string()}</div>
            </section>
        }
        .into_view(),
    };

    view! {
        <Transition>
            {page}
        </Transition>
    }
}

#[component]
pub fn UserPage() -> impl IntoView {
    let (auth_state, _) = auth_state();

    view! {
        <Show
            when=move || auth_state.get().is_logged_in()
            fallback={
                || view! {<LoggedOutView />}
            }
        >
            <Sidebar />
            <Outlet />
        </Show>
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    view! {
        <div class="text-white">"this moi"</div>
    }
}
