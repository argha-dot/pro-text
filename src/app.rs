use crate::utils::{AuthState, GlobalState};
use crate::{
    components::NoteMain,
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_query::*;
use leptos_query_devtools::LeptosQueryDevtools;
use leptos_router::*;

use crate::note::*;

#[component]
pub fn App() -> impl IntoView {
    let note_state = create_rw_signal(GlobalState {
        auth: AuthState::LoggedIn {
            username: "user_id".to_string(),
            username_hash: "".to_string(),
            password: "".to_string(),
        },
        current_note: None,
    });

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_query_client();
    provide_context(note_state);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/pro-text.css"/>
        <LeptosQueryDevtools />

        // sets the document title
        <Title text="Pro Text"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=HomePage>
                        <Route path=":id" view=NoteMain />
                        <Route path="" view=|| view! { <div>"Select a note"</div> } />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
