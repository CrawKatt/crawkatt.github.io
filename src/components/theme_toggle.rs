use leptos::prelude::*;
use crate::components::{MoonIcon, SunIcon};
use crate::context::use_theme_context;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme_ctx = use_theme_context();
    let is_dark = theme_ctx.is_dark;

    Effect::new(move |_| {
        let window = window();
        let document = window.document().unwrap();
        let html = document.document_element().unwrap();

        let stored_theme = window.local_storage().unwrap().unwrap().get_item("theme").unwrap();
        let prefers_dark = window.match_media("(prefers-color-scheme: dark)").unwrap().unwrap().matches();

        let dark_mode = stored_theme.as_deref() == Some("dark") || (stored_theme.is_none() && prefers_dark);
        is_dark.set(dark_mode);

        if dark_mode {
            html.class_list().add_1("dark").unwrap();
        } else {
            html.class_list().remove_1("dark").unwrap();
        }
    });

    let toggle_theme = move |_| {
        let window = window();
        let document = window.document().unwrap();
        let html = document.document_element().unwrap();

        let new_theme = if is_dark.get() { "light" } else { "dark" };
        window.local_storage().unwrap().unwrap().set_item("theme", new_theme).unwrap();

        is_dark.update(|dark| *dark = !*dark);

        if is_dark.get() {
            html.class_list().add_1("dark").unwrap();
        } else {
            html.class_list().remove_1("dark").unwrap();
        }
    };

    view! {
        <button
            class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 w-10 border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            on:click=toggle_theme
            aria-label="Toggle theme"
        >
            <Show when=move || is_dark.get() fallback=|| view! { <SunIcon /> }>
                <MoonIcon />
            </Show>
        </button>
    }
}
