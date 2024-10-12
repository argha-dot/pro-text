use leptos::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div
            class="flex flex-col gap-4 items-center justify-center
                h-screen w-full text-white"
        >
            <p>"404 Not Found"</p>
        </div>
    }
}
