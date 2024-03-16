use leptos::*;
use leptos_router::*;

use crate::models::NoteMetadata;
use crate::utils::{get_all_note_metadatas, set_current_note};
use crate::{api::AddNote, utils::use_get_note_metadatas};

#[component]
pub fn SideItem(
    /// The Metadata of the note item
    #[prop(into)]
    val: RwSignal<NoteMetadata>,
    /// The current selected note
    current_selected: Option<String>,
) -> impl IntoView {
    let note_title = move || {
        val.with(|val| {
            if val.title.is_empty() {
                view! {<p class="sidebar__item__val--untitled">"Untitled"</p>}.into_view()
            } else {
                view! {<p class="sidebar__item__val">{val.title.clone()}</p>}.into_view()
            }
        })
    };

    let class = match current_selected {
        Some(selected) => {
            if selected == val.get().id {
                format!("sidebar__item sidebar__item--selected")
            } else {
                format!("sidebar__item")
            }
        }
        None => format!("sidebar__item"),
    };

    let set_current = set_current_note();
    let on_click = move |_| set_current(Some(val.get().id));

    view! {
        <li on:click=on_click class=class>{note_title}</li>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    let add_note = create_server_multi_action::<AddNote>();

    let (note_state, set_note_state) = use_get_note_metadatas();
    let notes = move || note_state.get().note_metadatas;

    let _ = watch(
        move || add_note.version().get(),
        move |_, _, _| set_note_state(get_all_note_metadatas()),
        false,
    );

    view! {
        <aside class="sidebar">
            <ul>
                <Transition fallback=move || view! {<p>"Loading"</p>}>
                    {
                        move || notes().get().map(move |notes| match notes {
                            Err(e) => view! { <pre class="sidebar--error">"Server Error: " {e.to_string()}</pre> }.into_view(),
                            Ok(notes) => {
                                if notes.is_empty() {
                                    view! { <pre class="sidebar--no_notes">"No Notes"</pre> }.into_view()
                                } else {
                                    notes
                                        .into_iter()
                                        .map(move |note| {
                                            view! {
                                                <SideItem
                                                    val=note
                                                    current_selected=note_state.get().current_note
                                                />
                                            }
                                        })
                                        .collect_view()
                                }
                            }
                        })

                    }
                </Transition>
                <li>
                    <MultiActionForm class="sidebar__form" action=add_note>
                        <input class="sidebar__form__input" type="text" name="title" />

                        <button class="sidebar__form__button" type="submit">"+"</button>
                    </MultiActionForm>
                </li>
            </ul>
        </aside>
    }
}
