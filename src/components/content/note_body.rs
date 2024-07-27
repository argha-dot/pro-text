use leptos::{ev::Event, *};

#[component]
pub fn NoteBody(
    /// The Body of the Note
    #[prop(into)]
    body: RwSignal<String>,
) -> impl IntoView {
    let on_input = move |ev: Event| body.set(event_target_value(&ev));

    view! {
        <textarea
            name="note"
            class="note__body"
            prop:value=move || body.get()
            on:input=on_input
        >
            {body.get_untracked()}
        </textarea>
    }
}
