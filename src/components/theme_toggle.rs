use leptos::prelude::*;

use crate::components::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (is_dark, set_is_dark) = signal(false);
    Effect::new(move |_| {
        let document = window().document().unwrap();
        let html = document.document_element().unwrap();
        let has_dark_class = html.class_list().contains("dark");

        set_is_dark.set(has_dark_class);
    });

    let toggle_theme = move |_| {
        set_is_dark.update(|dark| *dark = !*dark);

        let document = window().document().unwrap();
        let html = document.document_element().unwrap();

        if is_dark.get() {
            let _ = html.class_list().add_1("dark");
        } else {
            let _ = html.class_list().remove_1("dark");
        }
    };

    view! {
        <button
            class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 w-10 border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            on:click=toggle_theme
            aria-label="Toggle theme"
        >
            <Show
                when=move || is_dark.get()
                fallback=|| view! { <SunIcon /> }
            >
                <MoonIcon />
            </Show>
        </button>
    }
}
