use crate::utils::{AuthState, GlobalState};
use crate::{
    components::NoteMain,
    error_template::{AppError, ErrorTemplate},
    pages::UserProfile,
};
use leptos::*;
use leptos_meta::*;
use leptos_query::*;
use leptos_query_devtools::LeptosQueryDevtools;
use leptos_router::*;

use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    let global_state = create_rw_signal(GlobalState {
        auth: AuthState::LoggedOut,
        current_note: None,
    });

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_query_client();
    provide_context(global_state);

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
            <main class="min-h-screen w-full bg-primary flex">
                <Routes>
                    // Logged Out Routes
                    <Route path="" view={|| view! {<Outlet />}}>
                        <Route path=":username" view={UserPage}>
                            <Route path="note" view=|| view! {<Outlet />}>
                                <Route path=":note_id" view=|| view! {<NoteMain />} />
                                <Route path="" view=|| view! {<div>"select note"</div>} />
                            </Route>
                            <Route path="" view={UserProfile} />
                        </Route>
                        <Route path="" view={HomePage} />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
