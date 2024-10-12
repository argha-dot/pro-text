use ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

#[component]
fn UserNameForm() -> impl IntoView {
    let username = create_rw_signal(String::new());

    let label_class = move || {
        format!(
            "login__form__input {}",
            if !username.get().is_empty() {
                "has-text"
            } else {
                ""
            }
        )
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        use_navigate()(format!("{}", username.get()).as_str(), Default::default());
    };

    view! {
        <Form
            method="GET"
            action=""
            class="login__form"
            on:submit=on_submit
        >
            <label
                html_for="username"
                class={label_class}
            >
                <p>"Username"</p>
                <input
                    type="text"
                    name="user"
                    id="username"
                    required={true}
                    value={username}
                    on:input=move |ev| {
                        username.set(event_target_value(&ev));
            }
                />
            </label>
            <button
                type="submit"
            >
                "NEXT"
            </button>
        </Form>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div
            class="flex flex-col gap-4 items-center justify-center
                h-screen w-full text-white"
        >
            <p>"Welcome to Another Text Editor App Thingy"</p>

            <UserNameForm />
        </div>
    }
}
