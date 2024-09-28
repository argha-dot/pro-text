use crate::models::NoteMetadata;
use crate::utils::set_current_note;

use ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn SideItem(
    /// The Metadata of the note item
    #[prop(into)]
    val: Signal<NoteMetadata>,
    /// The current selected note
    #[prop(into, default = None.into())]
    current_selected: MaybeSignal<Option<String>>,
    /// On Delete
    delete_note: Action<String, ()>,
    /// The User Id
    #[prop(into)]
    current_user: MaybeSignal<String>,
) -> impl IntoView {
    let note_title = move || {
        val.with(|val| {
            if val.title.is_empty() {
                view! {
                    <p class="sidebar__item__val--untitled">"Untitled"</p>
                }
                .into_view()
            } else {
                view! {
                    <p class="sidebar__item__val">{val.title.clone()}</p>
                }
                .into_view()
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

    let (_, set_current) = set_current_note();
    let on_click_note = move |_| set_current(Some(val.get().id));

    let on_click_delete = move |ev: SubmitEvent| {
        ev.prevent_default();
        delete_note.dispatch(val.get().id.clone())
    };

    view! {
        <li class=class>
            <A href=move || format!("{}/note/{}", current_user.get(), val.get().id) on:click=on_click_note>{note_title}</A>
            <form on:submit={on_click_delete}>
                <button class="sidebar__item__del">"X"</button>
            </form>
        </li>
    }
}
