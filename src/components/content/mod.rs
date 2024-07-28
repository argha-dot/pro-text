mod note_body;
mod note_title;

pub use self::note_body::*;
pub use self::note_title::*;

use ev::SubmitEvent;
use leptos::*;
use leptos_query::QueryResult;
use leptos_router::*;

use crate::api::update_note;
use crate::queries::get_all_note_metadatas_query;
use crate::queries::get_note_query;
use crate::queries::AllNoteMetadatasTag;
use crate::utils::get_username;

#[component]
pub fn NoteMain() -> impl IntoView {
    let params = use_params_map();
    let current_user = get_username();

    let note_id =
        Signal::derive(move || params.with(|params| params.get("id").cloned().unwrap_or_default()));

    let QueryResult { data, refetch, .. } =
        get_note_query().use_query(move || (note_id.get().clone(), current_user.get().unwrap()));

    let note_title = create_rw_signal("".to_string());
    let note_body = create_rw_signal("".to_string());

    create_effect(move |_| {
        data.get().map(|note| match note {
            Ok(note) => {
                note_title.set(note.title);
                note_body.set(note.note);
            }
            Err(_) => {}
        });
    });

    let update_todo = create_action(move |id: &String| {
        let id = id.clone();
        let refetch = refetch.clone();

        let note_query = get_note_query();
        let notes_query = get_all_note_metadatas_query();

        async move {
            note_query.cancel_query((id.clone(), current_user.get().unwrap()));
            notes_query.cancel_query((AllNoteMetadatasTag, current_user.get().unwrap()));

            let res = update_note(
                id.clone(),
                note_title.get(),
                note_body.get(),
                current_user.get().unwrap_or_default(),
            )
            .await;

            if res.is_ok() {
                note_query.invalidate_query((id.clone(), current_user.get().unwrap_or_default()));
                notes_query.invalidate_query((
                    AllNoteMetadatasTag,
                    current_user.get().unwrap_or_default(),
                ));
                refetch();
            }
        }
    });

    let on_save = move |ev: SubmitEvent| {
        ev.prevent_default();
        update_todo.dispatch(note_id.get());
    };

    view! {
        <form on:submit={on_save} class="note__container">
            <Transition fallback=move || view! {<p class="note__container__error">"Loading..."</p>}>
                {
                    move || data.get().map(|note| match note {
                        Err(e) => view! {
                            <pre class="note__container__error">
                                "Server Error: " {e.to_string()}
                            </pre>
                        }.into_view(),
                        Ok(note) => view! {
                            <input name="id" type="hidden" value={note.id} />
                            <NoteTitle title={note_title} />
                            <NoteBody body={note_body}/>
                        }
                        .into_view(),
                    })
                }
            </Transition>
        </form>
    }
}
