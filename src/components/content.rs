use leptos::{ev::Event, *};
use leptos_router::*;

use crate::api::get_note;

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
    let params = use_params_map();
    let note_id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let note = create_resource(note_id, |note_id| get_note(note_id));

    view! {
        <article class="note__container">
            <Transition fallback=move || view! {<p>"Loading..."</p>}>
                {
                    note.get().map(|note| match note {
                        Err(e) => view! {<pre>"Server Error: " {e.to_string()}</pre>}.into_view(),
                        Ok(note) => view! {
                            <NoteTitle title=note.title />
                            <NoteBody body=note.note/>
                        }
                        .into_view(),
                    })
                }
            </Transition>
        </article>
    }
}
