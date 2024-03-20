use leptos::{ev::Event, *};
use leptos_router::*;

use crate::{
    api::{get_note, SetTitle},
    utils::{get_all_note_metadatas, use_get_note_metadatas},
};

#[component]
fn NoteBody(
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

#[component]
pub fn NoteTitle(
    /// The Title of the note
    #[prop(into)]
    title: RwSignal<String>,
) -> impl IntoView {
    let on_input = move |ev: Event| title.set(event_target_value(&ev));

    view! {
        <div class="note__heading">
            <input
                name="title"
                on:input=on_input
                prop:value=title
                value=title.get_untracked()
            />
            <button type="submit">"SAVE"</button>
        </div>
    }
}

#[component]
pub fn NoteMain() -> impl IntoView {
    let params = use_params_map();

    let note_id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let note = create_resource(note_id, |note_id| get_note(note_id));

    let (_, set_note_state) = use_get_note_metadatas();
    let update_note = create_server_action::<SetTitle>();
    let response = update_note.value();

    create_effect(move |_| {
        if let Some(Ok(_)) = response.get() {
            set_note_state(get_all_note_metadatas());
        }
    });

    view! {
        <ActionForm action=update_note class="note__container">
            <Transition fallback=move || view! {<p>"Loading..."</p>}>
                {
                    note.get().map(|note| match note {
                        Err(e) => view! {<pre>"Server Error: " {e.to_string()}</pre>}.into_view(),
                        Ok(note) => view! {
                            <input name="id" type="hidden" value={note.id} />
                            <NoteTitle title=note.title />
                            <NoteBody body=note.note/>
                        }
                        .into_view(),
                    })
                }
            </Transition>
        </ActionForm>
    }
}
