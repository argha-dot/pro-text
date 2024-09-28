mod add_form;
mod item;
mod user_card;

pub use self::add_form::*;
pub use self::item::*;
pub use self::user_card::*;

use leptos::*;
use leptos_query::QueryResult;
use leptos_router::*;

use crate::api::delete_note;
use crate::queries::{get_all_note_metadatas_query, get_note_query, AllNoteMetadatasTag};
use crate::utils::{get_username, set_current_note};

#[component]
pub fn Sidebar() -> impl IntoView {
    let (current_note, set_current_note) = set_current_note();
    let current_user = get_username();

    let navigate = use_navigate();
    if current_user.get().is_none() {
        navigate("/login", Default::default());
    }

    let QueryResult { data, refetch, .. } = get_all_note_metadatas_query().use_query(move || {
        (
            AllNoteMetadatasTag,
            current_user.get().clone().unwrap_or_default(),
        )
    });

    let delete_note_action = create_action(move |id: &String| {
        let id = id.clone();
        let refetch = refetch.clone();

        let note_query = get_note_query();
        let notes_query = get_all_note_metadatas_query();

        async move {
            notes_query.cancel_query((AllNoteMetadatasTag, current_user.get().unwrap_or_default()));
            set_current_note.set(None);

            let _ = delete_note(id.clone(), current_user.get().unwrap_or_default()).await;

            let _ = note_query.invalidate_query((id, current_user.get().unwrap_or_default()));

            refetch();
        }
    });

    let notes_list = move || {
        data.get().map(|notes| match notes {
            Err(e) => view! {
                <pre class="sidebar--error">"Server Error: " {e.to_string()}</pre>
            }
            .into_view(),
            Ok(notes) => {
                if notes.is_empty() {
                    return view! { <pre class="sidebar--no_notes">"No Notes"</pre> }.into_view();
                }

                notes
                    .into_iter()
                    .map(|note| {
                        view! {
                            <SideItem
                                current_user={Signal::derive(move || current_user.get().unwrap_or_default())}
                                delete_note={delete_note_action}
                                val=Signal::derive(move || note.clone())
                                current_selected={Signal::derive(move || current_note.get())}
                            />
                        }
                        .into_view()
                    })
                    .collect_view()
            }
        })
    };

    view! {
        <aside class="sidebar">
            <UserSidebarCard />
            <ul>
                <Transition fallback=move || view! {<p style="background: red;">"Loading"</p>}>
                    {notes_list}
                    <AddFormComponent />
                </Transition>
            </ul>
        </aside>
    }
}
