use leptos::{ev::Event, *};

#[component]
fn NoteBody(
    /// The Body of the Note
    #[prop(into)]
    body: RwSignal<String>,
) -> impl IntoView {
    let on_input = move |ev: Event| body.set(event_target_value(&ev));

    view! {
        <textarea
            class="note__body"
            prop:value=move || body.get()
            on:input=on_input
        >
            {body.get_untracked()}
        </textarea>
    }
}

#[component]
pub fn NoteTitle(#[prop(into)] title: RwSignal<String>) -> impl IntoView {
    let on_input = move |ev: Event| title.set(event_target_value(&ev));

    view! {
        <input
            class="note__heading"
            on:input=on_input
            prop:value=title
            value=title.get_untracked()
        />
    }
}

#[component]
pub fn NoteMain() -> impl IntoView {
    let data = create_rw_signal("Hello".to_owned());
    let title = create_rw_signal("Title".to_owned());

    view! {
        <article class="note__container">
            <NoteTitle title=title />
            <NoteBody body=data/>
        </article>
    }
}
