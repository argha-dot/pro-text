use crate::api::AddNote;
use crate::queries::{get_all_note_metadatas_query, get_note_query, AllNoteMetadatasTag};
use crate::utils::get_username;
use leptos::html::Input;
use leptos::*;
use leptos_router::*;

#[component]
pub fn AddFormComponent() -> impl IntoView {
    let current_user = get_username();

    let add_note = create_server_action::<AddNote>();
    let input_element = create_node_ref::<Input>();

    let response = add_note.value();
    let _ = get_note_query();
    let notes_query = get_all_note_metadatas_query();

    create_effect(move |_| {
        if let Some(Ok(_)) = response.get() {
            notes_query.invalidate_query((AllNoteMetadatasTag, current_user.get().unwrap()));
            let node = input_element.get().expect("ref to be loaded");
            node.set_value("");
        }
    });

    view! {
        <li>
            <ActionForm
                class="sidebar__form"
                action=add_note
            >
                {move || view! {
                    <input type="hidden" name="user_id" value={current_user.get().unwrap_or_default()} />
                }}
                <input _ref=input_element class="sidebar__form__input" type="text" name="title" />

                <button class="sidebar__form__button" type="submit">"+"</button>
            </ActionForm>
        </li>
    }
}
