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
    #[prop(into, default = None.into())]
    current_selected: MaybeSignal<Option<String>>,
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

    let class = move || match current_selected.get() {
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
        <li class=class>
            <A href=move || val.get().id on:click=on_click>{note_title}</A>
        </li>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    let add_note = create_server_multi_action::<AddNote>();

    let (note_state, set_note_state) = use_get_note_metadatas();
    let notes = move || note_state.get().note_metadatas;
    let input_element = create_node_ref();
    let set_current = set_current_note();

    let _ = watch(
        move || add_note.version().get(),
        move |_, _, _| {
            let node: HtmlElement<html::Input> = input_element.get().expect("input_ref not loaded");
            set_note_state(get_all_note_metadatas());
            set_current(Some(node.value()));
            node.set_value("");
        },
        false,
    );

    view! {
        <aside class="sidebar">
            <ul>
                <Transition fallback=move || view! {<p style="background: red;">"Loading"</p>}>
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
                    <li>
                        <MultiActionForm
                            class="sidebar__form"
                            action=add_note
                        >
                            <input _ref=input_element class="sidebar__form__input" type="text" name="title" />

                            <button class="sidebar__form__button" type="submit">"+"</button>
                        </MultiActionForm>
                    </li>
                </Transition>
            </ul>
        </aside>
    }
}
