use leptos::{ev::Event, *};

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
            <div>
                <button type="submit">"SAVE"</button>
            </div>
        </div>
    }
}
