use crate::api::AddNote;
use crate::queries::{get_all_note_metadatas_query, get_note_query, AllNoteMetadatasTag};
use leptos::*;
use leptos_router::*;

#[component]
pub fn AddFormComponent() -> impl IntoView {
    let add_note = create_server_action::<AddNote>();
    let input_element = create_node_ref();

    let response = add_note.value();
    let _ = get_note_query();
    let notes_query = get_all_note_metadatas_query();

    create_effect(move |_| {
        if let Some(Ok(_)) = response.get() {
            notes_query.invalidate_query(AllNoteMetadatasTag);
        }
    });

    view! {
        <li>
            <ActionForm
                class="sidebar__form"
                action=add_note
            >
                <input _ref=input_element class="sidebar__form__input" type="text" name="title" />

                <button class="sidebar__form__button" type="submit">"+"</button>
            </ActionForm>
        </li>
    }
}
